package slack

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/nexus/gateway/internal/bus"
	"github.com/nexus/gateway/internal/session"
)

type Config struct {
	Enabled      bool   `json:"enabled"`
	BotToken     string `json:"bot_token"`
	AppToken     string `json:"app_token"`
	SigningSecret string `json:"signing_secret"`
}

type SlackChannel struct {
	config  Config
	bus     *bus.Bus
	session *session.Manager
	http    *http.Client
	mu      sync.Mutex
	stopCh  chan struct{}
	wsURL   string
}

type slackEvent struct {
	Token       string          `json:"token"`
	TeamID      string          `json:"team_id"`
	APIAppID    string          `json:"api_app_id"`
	Event       json.RawMessage `json:"event"`
	Type        string          `json:"type"`
	EventID     string          `json:"event_id"`
	EventTime   int64           `json:"event_time"`
	Challenge   string          `json:"challenge,omitempty"`
}

type messageEvent struct {
	Type      string `json:"type"`
	User      string `json:"user"`
	Text      string `json:"text"`
	Channel   string `json:"channel"`
	EventTS   string `json:"event_ts"`
	BotID     string `json:"bot_id,omitempty"`
	Subtype   string `json:"subtype,omitempty"`
}

type appsConnectionsResponse struct {
	OK  bool   `json:"ok"`
	URL string `json:"url"`
}

func NewChannel(cfg Config) *SlackChannel {
	return &SlackChannel{
		config:  cfg,
		session: session.NewManager(),
		http:    &http.Client{Timeout: 10 * time.Second},
		stopCh:  make(chan struct{}),
	}
}

func (s *SlackChannel) Name() string {
	return "slack"
}

func (s *SlackChannel) Start(b *bus.Bus) error {
	if !s.config.Enabled {
		log.Println("[slack] Channel disabled, skipping")
		return nil
	}
	s.bus = b
	go s.run()
	return nil
}

func (s *SlackChannel) Stop() error {
	close(s.stopCh)
	return nil
}

func (s *SlackChannel) run() {
	log.Println("[slack] Starting Socket Mode connection")

	for {
		select {
		case <-s.stopCh:
			return
		default:
		}

		if err := s.connectSocketMode(); err != nil {
			log.Printf("[slack] Connection error: %v, retrying in 5s", err)
			time.Sleep(5 * time.Second)
		}
	}
}

func (s *SlackChannel) connectSocketMode() error {
	resp, err := s.http.Post(
		"https://slack.com/api/apps.connections.connect",
		"application/json",
		strings.NewReader(`{}`),
	)
	if err != nil {
		return fmt.Errorf("apps.connections.connect failed: %w", err)
	}
	defer resp.Body.Close()

	var connResp appsConnectionsResponse
	if err := json.NewDecoder(resp.Body).Decode(&connResp); err != nil {
		return fmt.Errorf("decode response failed: %w", err)
	}

	if !connResp.OK || connResp.URL == "" {
		return fmt.Errorf("slack API error: ok=%v url=%s", connResp.OK, connResp.URL)
	}

	ws, _, err := websocket.DefaultDialer.Dial(connResp.URL, nil)
	if err != nil {
		return fmt.Errorf("websocket dial failed: %w", err)
	}
	defer ws.Close()

	log.Println("[slack] Connected via Socket Mode")

	for {
		select {
		case <-s.stopCh:
			return nil
		default:
		}

		_, msg, err := ws.ReadMessage()
		if err != nil {
			return fmt.Errorf("read error: %w", err)
		}

		var event slackEvent
		if err := json.Unmarshal(msg, &event); err != nil {
			continue
		}

		switch event.Type {
		case "events_api":
			s.handleEvent(event.Event)
		case "url_verification":
			s.handleChallenge(ws, event.Challenge)
		}
	}
}

func (s *SlackChannel) handleEvent(data json.RawMessage) {
	var msg messageEvent
	if err := json.Unmarshal(data, &msg); err != nil {
		return
	}

	if msg.User == "" || msg.BotID != "" {
		return
	}

	sess := s.session.GetOrCreate("slack", msg.User)

	s.bus.PublishAsync("message:"+sess.ID, bus.Message{
		Channel:   "slack",
		UserID:    msg.User,
		SessionID: sess.ID,
		Content:   msg.Text,
	})

	log.Printf("[slack] Message from %s: %.50s", msg.User, msg.Text)
}

func (s *SlackChannel) handleChallenge(ws *websocket.Conn, challenge string) {
	payload := map[string]string{"challenge": challenge}
	ws.WriteJSON(payload)
	log.Println("[slack] Challenge answered")
}

func (s *SlackChannel) SendMessage(channel, text string) error {
	body := map[string]string{
		"channel": channel,
		"text":    text,
	}
	data, _ := json.Marshal(body)

	req, err := http.NewRequest("POST", "https://slack.com/api/chat.postMessage",
		strings.NewReader(string(data)))
	if err != nil {
		return err
	}
	req.Header.Set("Authorization", "Bearer "+s.config.BotToken)
	req.Header.Set("Content-Type", "application/json")

	resp, err := s.http.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("slack API error: %d", resp.StatusCode)
	}
	return nil
}
