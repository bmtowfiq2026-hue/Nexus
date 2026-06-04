package msteams

import "github.com/nexus/gateway/internal/channel/webhook"

type Config struct {
	Enabled    bool   `json:"enabled"`
	WebhookURL string `json:"webhook_url"`
	AppID      string `json:"app_id"`
	AppSecret  string `json:"app_secret"`
}

type Channel struct{ *webhook.Channel }

func NewChannel(cfg Config) *Channel {
	return &Channel{Channel: webhook.New(webhook.Config{
		Name: "msteams", Enabled: cfg.Enabled, WebhookURL: cfg.WebhookURL,
	})}
}
