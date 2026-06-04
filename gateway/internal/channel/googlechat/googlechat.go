package googlechat

import "github.com/nexus/gateway/internal/channel/webhook"

type Config struct {
	Enabled     bool   `json:"enabled"`
	WebhookURL  string `json:"webhook_url"`
	SpaceID     string `json:"space_id"`
}

type Channel struct{ *webhook.Channel }

func NewChannel(cfg Config) *Channel {
	return &Channel{Channel: webhook.New(webhook.Config{
		Name: "googlechat", Enabled: cfg.Enabled, WebhookURL: cfg.WebhookURL,
	})}
}
