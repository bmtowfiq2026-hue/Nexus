package session

import (
	"sync"
	"time"

	"github.com/google/uuid"
)

type Session struct {
	ID        string            `json:"id"`
	Channel   string            `json:"channel"`
	UserID    string            `json:"user_id"`
	History   []Turn            `json:"history"`
	Metadata  map[string]string `json:"metadata,omitempty"`
	CreatedAt time.Time         `json:"created_at"`
	UpdatedAt time.Time         `json:"updated_at"`
}

type Turn struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type Manager struct {
	mu       sync.RWMutex
	sessions map[string]*Session
}

func NewManager() *Manager {
	return &Manager{
		sessions: make(map[string]*Session),
	}
}

func (m *Manager) GetOrCreate(channel, userID string) *Session {
	key := channel + ":" + userID

	m.mu.RLock()
	s, ok := m.sessions[key]
	m.mu.RUnlock()

	if ok {
		return s
	}

	m.mu.Lock()
	defer m.mu.Unlock()

	if s, ok := m.sessions[key]; ok {
		return s
	}

	s = &Session{
		ID:        uuid.New().String(),
		Channel:   channel,
		UserID:    userID,
		History:   make([]Turn, 0),
		Metadata:  make(map[string]string),
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	}
	m.sessions[key] = s
	return s
}

func (m *Manager) Get(sessionID string) (*Session, bool) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	s, ok := m.sessions[sessionID]
	return s, ok
}

func (m *Manager) AddTurn(sessionID, role, content string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if s, ok := m.sessions[sessionID]; ok {
		s.History = append(s.History, Turn{Role: role, Content: content})
		s.UpdatedAt = time.Now()
	}
}
