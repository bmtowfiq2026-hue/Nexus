package whatsapp

import "github.com/nexus/gateway/internal/channel/webhook"

type Config struct {
	Enabled     bool   `json:"enabled"`
	WebhookPath string `json:"webhook_path"`
	VerifyToken string `json:"verify_token"`
	APIToken    string `json:"api_token"`
}

type Channel struct {
	*webhook.Channel
}

func NewChannel(cfg Config) *Channel {
	return &Channel{Channel: webhook.New(webhook.Config{
		Name: "whatsapp", Enabled: cfg.Enabled,
		WebhookPath: cfg.WebhookPath, APIToken: cfg.APIToken,
	})}
}
