package signal

import "github.com/nexus/gateway/internal/channel/webhook"

type Config struct {
	Enabled     bool   `json:"enabled"`
	PhoneNumber string `json:"phone_number"`
	SignalAPI   string `json:"signal_api"`
}

type Channel struct{ *webhook.Channel }

func NewChannel(cfg Config) *Channel {
	return &Channel{Channel: webhook.New(webhook.Config{
		Name: "signal", Enabled: cfg.Enabled,
		PollURL: cfg.SignalAPI + "/v1/receive/" + cfg.PhoneNumber,
		PollInterval: 5,
	})}
}
