package matrix

import "github.com/nexus/gateway/internal/channel/webhook"

type Config struct {
	Enabled    bool   `json:"enabled"`
	Homeserver string `json:"homeserver"`
	AccessToken string `json:"access_token"`
	RoomID     string `json:"room_id"`
}

type Channel struct {
	*webhook.Channel
	config Config
}

func NewChannel(cfg Config) *Channel {
	wc := webhook.New(webhook.Config{
		Name:       "matrix",
		Enabled:    cfg.Enabled,
		APIToken:   cfg.AccessToken,
		PollURL:    cfg.Homeserver + "/_matrix/client/v3/sync?access_token=" + cfg.AccessToken,
		PollInterval: 10,
	})
	return &Channel{Channel: wc, config: cfg}
}
