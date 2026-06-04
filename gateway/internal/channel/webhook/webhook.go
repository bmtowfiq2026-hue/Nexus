package webhook

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"strings"
	"time"

	"github.com/nexus/gateway/internal/bus"
)

type Config struct {
	Name       string `json:"name"`
	Enabled    bool   `json:"enabled"`
	WebhookURL string `json:"webhook_url,omitempty"`
	APIToken   string `json:"api_token,omitempty"`
	BotToken   string `json:"bot_token,omitempty"`
	WebhookPath string `json:"webhook_path,omitempty"`
	PollURL    string `json:"poll_url,omitempty"`
	PollInterval int   `json:"poll_interval,omitempty"`
}

type Channel struct {
	config Config
	bus    *bus.Bus
	client *http.Client
	stopCh chan struct{}
}

func New(cfg Config) *Channel {
	return &Channel{
		config:  cfg,
		client:  &http.Client{Timeout: 30 * time.Second},
		stopCh:  make(chan struct{}),
	}
}

func (c *Channel) Name() string { return c.config.Name }

func (c *Channel) Start(b *bus.Bus) error {
	if !c.config.Enabled {
		log.Printf("[%s] Disabled", c.config.Name)
		return nil
	}
	c.bus = b
	log.Printf("[%s] Ready", c.config.Name)

	if c.config.PollURL != "" {
		go c.pollLoop()
	}
	return nil
}

func (c *Channel) Stop() error {
	close(c.stopCh)
	return nil
}

func (c *Channel) SendMessage(content string) error {
	if c.config.WebhookURL == "" {
		return nil
	}
	body := map[string]string{"text": content}
	jsonBody, _ := json.Marshal(body)
	resp, err := c.client.Post(c.config.WebhookURL, "application/json", strings.NewReader(string(jsonBody)))
	if err != nil {
		return fmt.Errorf("webhook send error: %w", err)
	}
	defer resp.Body.Close()
	io.Copy(io.Discard, resp.Body)
	return nil
}

func (c *Channel) HandleWebhook(rw http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(rw, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	body, _ := io.ReadAll(r.Body)
	var msg struct {
		Message  string `json:"message"`
		Text     string `json:"text"`
		Content  string `json:"content"`
		UserID   string `json:"user_id"`
		SenderID string `json:"sender_id"`
		From     string `json:"from"`
	}
	json.Unmarshal(body, &msg)

	content := msg.Message
	if content == "" { content = msg.Text }
	if content == "" { content = msg.Content }

	userID := msg.UserID
	if userID == "" { userID = msg.SenderID }
	if userID == "" { userID = msg.From }
	if userID == "" { userID = "unknown" }

	c.bus.PublishAsync("message:"+userID, bus.Message{
		Channel: c.config.Name, UserID: userID, SessionID: userID, Content: content,
	})
	json.NewEncoder(rw).Encode(map[string]string{"status": "ok"})
}

func (c *Channel) RegisterWebhook(mux *http.ServeMux) {
	path := c.config.WebhookPath
	if path == "" {
		path = "/webhook/" + c.config.Name
	}
	mux.HandleFunc(path, c.HandleWebhook)
}

func (c *Channel) pollLoop() {
	interval := c.config.PollInterval
	if interval < 1 { interval = 5 }
	ticker := time.NewTicker(time.Duration(interval) * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-c.stopCh:
			return
		case <-ticker.C:
			c.poll()
		}
	}
}

func (c *Channel) poll() {
	if c.config.PollURL == "" { return }
	resp, err := c.client.Get(c.config.PollURL)
	if err != nil {
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	var updates []struct {
		ID      string `json:"id"`
		Text    string `json:"text"`
		Message string `json:"message"`
		From    string `json:"from"`
		UserID  string `json:"user_id"`
	}
	json.Unmarshal(body, &updates)
	for _, u := range updates {
		content := u.Message
		if content == "" { content = u.Text }
		if content != "" {
			c.bus.PublishAsync("message:"+u.ID, bus.Message{
				Channel: c.config.Name, UserID: u.From,
				SessionID: u.ID, Content: content,
			})
		}
	}
}
