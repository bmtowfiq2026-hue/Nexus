package twilio

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/nexus/gateway/internal/channel/webhook"
)

type Config struct {
	Enabled    bool   `json:"enabled"`
	AccountSID string `json:"account_sid"`
	AuthToken  string `json:"auth_token"`
	FromNumber string `json:"from_number"`
}

type Channel struct {
	*webhook.Channel
}

func NewChannel(cfg Config) *Channel {
	return &Channel{Channel: webhook.New(webhook.Config{
		Name: "twilio", Enabled: cfg.Enabled, WebhookPath: "/webhook/twilio",
	})}
}

func (c *Channel) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/webhook/twilio", c.handleSMS)
}

func (c *Channel) handleSMS(rw http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(rw, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	r.ParseForm()
	from := r.FormValue("From")
	content := r.FormValue("Body")
	if content != "" && from != "" {
		log.Printf("[twilio] SMS from %s: %s", from, content)
	}
	json.NewEncoder(rw).Encode(map[string]string{"status": "ok"})
}
