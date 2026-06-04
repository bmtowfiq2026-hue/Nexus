package line

import "github.com/nexus/gateway/internal/channel/webhook"

type Config struct {
	Enabled    bool   `json:"enabled"`
	ChannelSecret string `json:"channel_secret"`
	ChannelToken  string `json:"channel_token"`
}

type Channel struct{ *webhook.Channel }

func NewChannel(cfg Config) *Channel {
	return &Channel{Channel: webhook.New(webhook.Config{
		Name: "line", Enabled: cfg.Enabled, APIToken: cfg.ChannelToken,
		WebhookPath: "/webhook/line",
	})}
}
