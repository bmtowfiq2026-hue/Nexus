package discord

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
	Enabled  bool   `json:"enabled"`
	BotToken string `json:"bot_token"`
}

type DiscordChannel struct {
	config  Config
	bus     *bus.Bus
	session *session.Manager
	ws      *websocket.Conn
	http    *http.Client
	mu      sync.Mutex
	stopCh  chan struct{}
	seq     int64
	gwURL   string
}

type gatewayPayload struct {
	Op int             `json:"op"`
	D  json.RawMessage `json:"d,omitempty"`
	S  *int64          `json:"s,omitempty"`
	T  *string         `json:"t,omitempty"`
}

type identifyData struct {
	Token      string            `json:"token"`
	Intents    int               `json:"intents"`
	Properties map[string]string `json:"properties"`
}

type messageCreate struct {
	ID        string `json:"id"`
	ChannelID string `json:"channel_id"`
	Author    struct {
		ID            string `json:"id"`
		Username      string `json:"username"`
		Discriminator string `json:"discriminator"`
		Bot           bool   `json:"bot"`
	} `json:"author"`
	Content string `json:"content"`
}

const (
	gatewayURL  = "wss://gateway.discord.gg/?v=10&encoding=json"
	intentGuildMessages = 1 << 9
	intentDirectMessages = 1 << 12
	intentMessageContent = 1 << 15
)

func NewChannel(cfg Config) *DiscordChannel {
	return &DiscordChannel{
		config:  cfg,
		session: session.NewManager(),
		http:    &http.Client{Timeout: 10 * time.Second},
		stopCh:  make(chan struct{}),
	}
}

func (d *DiscordChannel) Name() string {
	return "discord"
}

func (d *DiscordChannel) Start(b *bus.Bus) error {
	if !d.config.Enabled {
		log.Println("[discord] Channel disabled, skipping")
		return nil
	}
	d.bus = b
	go d.run()
	return nil
}

func (d *DiscordChannel) Stop() error {
	close(d.stopCh)
	if d.ws != nil {
		d.ws.Close()
	}
	return nil
}

func (d *DiscordChannel) run() {
	log.Println("[discord] Connecting to gateway...")

	for {
		select {
		case <-d.stopCh:
			return
		default:
		}

		if err := d.connect(); err != nil {
			log.Printf("[discord] Connection error: %v, retrying in 5s", err)
			time.Sleep(5 * time.Second)
			continue
		}

		if err := d.listen(); err != nil {
			log.Printf("[discord] Listen error: %v, reconnecting", err)
			time.Sleep(time.Second)
		}
	}
}

func (d *DiscordChannel) connect() error {
	d.mu.Lock()
	defer d.mu.Unlock()

	url := d.gwURL
	if url == "" {
		url = gatewayURL
	}

	ws, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		return fmt.Errorf("dial failed: %w", err)
	}
	d.ws = ws

	_, hello, err := ws.ReadMessage()
	if err != nil {
		return fmt.Errorf("hello read failed: %w", err)
	}

	var payload gatewayPayload
	if err := json.Unmarshal(hello, &payload); err != nil {
		return fmt.Errorf("hello parse failed: %w", err)
	}

	identify := identifyData{
		Token:   d.config.BotToken,
		Intents: intentGuildMessages | intentDirectMessages | intentMessageContent,
		Properties: map[string]string{
			"os":      "windows",
			"browser": "nexus",
			"device":  "nexus-gateway",
		},
	}
	identBytes, _ := json.Marshal(identify)

	identPayload := gatewayPayload{
		Op: 2,
		D:  identBytes,
	}

	if err := ws.WriteJSON(identPayload); err != nil {
		return fmt.Errorf("identify failed: %w", err)
	}

	log.Println("[discord] Connected and identified")
	return nil
}

func (d *DiscordChannel) listen() error {
	for {
		select {
		case <-d.stopCh:
			return nil
		default:
		}

		_, msg, err := d.ws.ReadMessage()
		if err != nil {
			return fmt.Errorf("read error: %w", err)
		}

		var payload gatewayPayload
		if err := json.Unmarshal(msg, &payload); err != nil {
			continue
		}

		if payload.S != nil {
			d.seq = *payload.S
		}

		switch payload.Op {
		case 0: // Dispatch
			if payload.T != nil && *payload.T == "MESSAGE_CREATE" {
				d.handleMessage(payload.D)
			}
		case 7: // Reconnect
			log.Println("[discord] Reconnect requested")
			return fmt.Errorf("reconnect requested")
		case 9: // Invalid session
			log.Println("[discord] Invalid session, reconnecting")
			d.gwURL = ""
			return fmt.Errorf("invalid session")
		}
	}
}

func (d *DiscordChannel) handleMessage(data json.RawMessage) {
	var msg messageCreate
	if err := json.Unmarshal(data, &msg); err != nil {
		return
	}

	if msg.Author.Bot {
		return
	}

	sess := d.session.GetOrCreate("discord", msg.Author.ID)

	d.bus.PublishAsync("message:"+sess.ID, bus.Message{
		Channel:   "discord",
		UserID:    msg.Author.ID,
		SessionID: sess.ID,
		Content:   msg.Content,
	})

	log.Printf("[discord] Message from %s: %.50s", msg.Author.Username, msg.Content)
}

func (d *DiscordChannel) SendMessage(channelID, content string) error {
	url := fmt.Sprintf("https://discord.com/api/v10/channels/%s/messages", channelID)
	body := map[string]string{"content": content}
	data, _ := json.Marshal(body)

	req, err := http.NewRequest("POST", url, strings.NewReader(string(data)))
	if err != nil {
		return err
	}
	req.Header.Set("Authorization", "Bot "+d.config.BotToken)
	req.Header.Set("Content-Type", "application/json")

	resp, err := d.http.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated {
		return fmt.Errorf("discord API error: %d", resp.StatusCode)
	}
	return nil
}
