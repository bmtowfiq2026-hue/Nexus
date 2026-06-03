package bus

import (
	"encoding/json"
	"log"
	"sync"
)

type Message struct {
	Channel   string          `json:"channel"`
	UserID    string          `json:"user_id"`
	SessionID string          `json:"session_id"`
	Content   string          `json:"content"`
	Metadata  json.RawMessage `json:"metadata,omitempty"`
}

type Response struct {
	SessionID string `json:"session_id"`
	Content   string `json:"content"`
	Error     string `json:"error,omitempty"`
}

type Handler func(msg Message) (Response, error)

type Bus struct {
	mu       sync.RWMutex
	handlers map[string][]Handler
}

func New() *Bus {
	return &Bus{
		handlers: make(map[string][]Handler),
	}
}

func (b *Bus) Subscribe(topic string, handler Handler) {
	b.mu.Lock()
	defer b.mu.Unlock()
	b.handlers[topic] = append(b.handlers[topic], handler)
	log.Printf("[bus] Subscribed to '%s' (total handlers: %d)", topic, len(b.handlers[topic]))
}

func (b *Bus) Publish(topic string, msg Message) ([]Response, error) {
	b.mu.RLock()
	handlers, ok := b.handlers[topic]
	b.mu.RUnlock()

	if !ok || len(handlers) == 0 {
		return nil, nil
	}

	var responses []Response
	for _, handler := range handlers {
		resp, err := handler(msg)
		if err != nil {
			log.Printf("[bus] Handler error on '%s': %v", topic, err)
			responses = append(responses, Response{Error: err.Error()})
			continue
		}
		responses = append(responses, resp)
	}
	return responses, nil
}

func (b *Bus) PublishAsync(topic string, msg Message) {
	go func() {
		if _, err := b.Publish(topic, msg); err != nil {
			log.Printf("[bus] Async publish error on '%s': %v", topic, err)
		}
	}()
}
