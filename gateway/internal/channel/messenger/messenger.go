package messenger

import "github.com/nexus/gateway/internal/channel/webhook"

type Config struct {
	Enabled      bool   `json:"enabled"`
	PageID       string `json:"page_id"`
	AccessToken  string `json:"access_token"`
	VerifyToken  string `json:"verify_token"`
}

type Channel struct{ *webhook.Channel }

func NewChannel(cfg Config) *Channel {
	return &Channel{Channel: webhook.New(webhook.Config{
		Name: "messenger", Enabled: cfg.Enabled, APIToken: cfg.AccessToken,
		WebhookPath: "/webhook/messenger",
	})}
}
