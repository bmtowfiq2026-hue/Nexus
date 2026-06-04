package webchat

import (
	"encoding/json"
	"log"
	"net/http"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/gorilla/websocket"
	"github.com/nexus/gateway/internal/bus"
	"github.com/nexus/gateway/internal/session"
)

var upgrader = websocket.Upgrader{
	CheckOrigin:     func(r *http.Request) bool { return true },
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
}

type Config struct {
	Enabled bool   `json:"enabled"`
	Path    string `json:"path"`
}

type wsClient struct {
	conn      *websocket.Conn
	sessionID string
	userID    string
	send      chan []byte
}

type WebChatChannel struct {
	config  Config
	bus     *bus.Bus
	session *session.Manager
	clients map[string]*wsClient
	mu      sync.RWMutex
}

func NewChannel(cfg Config) *WebChatChannel {
	return &WebChatChannel{
		config:  cfg,
		clients: make(map[string]*wsClient),
		session: session.NewManager(),
	}
}

func (w *WebChatChannel) Name() string { return "webchat" }

func (w *WebChatChannel) Start(b *bus.Bus) error {
	if !w.config.Enabled {
		log.Println("[webchat] Disabled, skipping")
		return nil
	}
	w.bus = b

	b.Subscribe("agent:response", func(msg bus.Message) (bus.Response, error) {
		w.mu.RLock()
		defer w.mu.RUnlock()
		for _, c := range w.clients {
			if c.sessionID == msg.SessionID {
				resp, _ := json.Marshal(map[string]string{
					"type":    "response",
					"content": msg.Content,
				})
				select {
				case c.send <- resp:
				default:
				}
			}
		}
		return bus.Response{SessionID: msg.SessionID, Content: msg.Content}, nil
	})

	log.Println("[webchat] Ready")
	return nil
}

func (w *WebChatChannel) Stop() error {
	w.mu.Lock()
	defer w.mu.Unlock()
	for _, c := range w.clients {
		close(c.send)
		c.conn.Close()
	}
	return nil
}

func (w *WebChatChannel) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/", w.handleRoot)
	mux.HandleFunc("/api/chat", w.handleREST)
	mux.HandleFunc("/ws", w.handleWS)
}

func (w *WebChatChannel) handleRoot(rw http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/" {
		http.NotFound(rw, r)
		return
	}
	rw.Header().Set("Content-Type", "text/html; charset=utf-8")
	rw.WriteHeader(http.StatusOK)
	rw.Write([]byte(ChatHTML))
}

func (w *WebChatChannel) handleREST(rw http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(rw, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var msg struct {
		SessionID string `json:"session_id"`
		Content   string `json:"content"`
	}
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(rw, "Invalid body", http.StatusBadRequest)
		return
	}
	sess, ok := w.session.Get(msg.SessionID)
	if !ok {
		sess = w.session.GetOrCreate("webchat", msg.SessionID)
	}
	w.bus.PublishAsync("message:incoming", bus.Message{
		Channel: "webchat", UserID: sess.UserID, SessionID: sess.ID, Content: msg.Content,
	})
	json.NewEncoder(rw).Encode(map[string]string{"session_id": sess.ID, "status": "received"})
}

func (w *WebChatChannel) handleWS(rw http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(rw, r, nil)
	if err != nil {
		log.Printf("[webchat] Upgrade error: %v", err)
		return
	}
	userID := uuid.New().String()
	sess := w.session.GetOrCreate("webchat", userID)
	client := &wsClient{
		conn: conn, sessionID: sess.ID, userID: userID,
		send: make(chan []byte, 256),
	}
	w.mu.Lock()
	w.clients[userID] = client
	w.mu.Unlock()
	log.Printf("[webchat] Client connected: %s (session: %s)", userID, sess.ID)
	go w.writePump(client)
	go w.readPump(client, sess)
}

func (w *WebChatChannel) readPump(client *wsClient, sess *session.Session) {
	defer func() {
		w.mu.Lock()
		delete(w.clients, client.userID)
		w.mu.Unlock()
		client.conn.Close()
	}()
	client.conn.SetReadLimit(65536)
	client.conn.SetReadDeadline(time.Now().Add(60 * time.Second))
	client.conn.SetPongHandler(func(string) error {
		client.conn.SetReadDeadline(time.Now().Add(60 * time.Second))
		return nil
	})
	for {
		_, msg, err := client.conn.ReadMessage()
		if err != nil {
			break
		}
		var incoming struct {
			Type    string `json:"type"`
			Content string `json:"content"`
		}
		if err := json.Unmarshal(msg, &incoming); err != nil {
			continue
		}
		if incoming.Type == "ping" {
			client.send <- []byte(`{"type":"pong"}`)
			continue
		}
		w.bus.PublishAsync("message:incoming", bus.Message{
			Channel: "webchat", UserID: client.userID,
			SessionID: sess.ID, Content: incoming.Content,
		})
	}
}

func (w *WebChatChannel) writePump(client *wsClient) {
	ticker := time.NewTicker(30 * time.Second)
	defer func() {
		ticker.Stop()
		client.conn.Close()
	}()
	for {
		select {
		case msg, ok := <-client.send:
			if !ok {
				client.conn.WriteMessage(websocket.CloseMessage, []byte{})
				return
			}
			client.conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
			if err := client.conn.WriteMessage(websocket.TextMessage, msg); err != nil {
				return
			}
		case <-ticker.C:
			client.conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
			if err := client.conn.WriteMessage(websocket.PingMessage, nil); err != nil {
				return
			}
		}
	}
}
