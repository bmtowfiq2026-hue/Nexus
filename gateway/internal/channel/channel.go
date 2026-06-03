package channel

import "github.com/nexus/gateway/internal/bus"

type Message struct {
	Channel   string `json:"channel"`
	UserID    string `json:"user_id"`
	SessionID string `json:"session_id"`
	Content   string `json:"content"`
}

type Response struct {
	SessionID string `json:"session_id"`
	Content   string `json:"content"`
	Error     string `json:"error,omitempty"`
}

type Channel interface {
	Name() string
	Start(b *bus.Bus) error
	Stop() error
}

type Registry struct {
	channels map[string]Channel
}

func NewRegistry() *Registry {
	return &Registry{
		channels: make(map[string]Channel),
	}
}

func (r *Registry) Register(name string, ch Channel) {
	r.channels[name] = ch
}

func (r *Registry) Get(name string) (Channel, bool) {
	ch, ok := r.channels[name]
	return ch, ok
}

func (r *Registry) List() []string {
	var names []string
	for name := range r.channels {
		names = append(names, name)
	}
	return names
}

func (r *Registry) StartAll(b *bus.Bus) error {
	for _, ch := range r.channels {
		if err := ch.Start(b); err != nil {
			return err
		}
	}
	return nil
}

func (r *Registry) StopAll() error {
	for _, ch := range r.channels {
		if err := ch.Stop(); err != nil {
			return err
		}
	}
	return nil
}
