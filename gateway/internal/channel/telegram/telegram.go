package telegram

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/nexus/gateway/internal/bus"
	"github.com/nexus/gateway/internal/session"
)

type Config struct {
	Enabled  bool   `json:"enabled"`
	BotToken string `json:"bot_token"`
}

type TelegramChannel struct {
	config  Config
	bus     *bus.Bus
	session *session.Manager
	http    *http.Client
	mu      sync.Mutex
	stopCh  chan struct{}
	offset  int
	baseURL string
}

type update struct {
	UpdateID int     `json:"update_id"`
	Message  *message `json:"message,omitempty"`
}

type message struct {
	MessageID int    `json:"message_id"`
	Chat      chat   `json:"chat"`
	From      user   `json:"from"`
	Text      string `json:"text"`
}

type chat struct {
	ID   int64  `json:"id"`
	Type string `json:"type"`
}

type user struct {
	ID           int64  `json:"id"`
	IsBot        bool   `json:"is_bot"`
	FirstName    string `json:"first_name"`
	Username     string `json:"username,omitempty"`
}

type sendMessageReq struct {
	ChatID int64  `json:"chat_id"`
	Text   string `json:"text"`
}

func NewChannel(cfg Config) *TelegramChannel {
	return &TelegramChannel{
		config:  cfg,
		session: session.NewManager(),
		http:    &http.Client{Timeout: 30 * time.Second},
		stopCh:  make(chan struct{}),
	}
}

func (t *TelegramChannel) Name() string {
	return "telegram"
}

func (t *TelegramChannel) Start(b *bus.Bus) error {
	if !t.config.Enabled {
		log.Println("[telegram] Channel disabled, skipping")
		return nil
	}
	t.bus = b
	t.baseURL = fmt.Sprintf("https://api.telegram.org/bot%s", t.config.BotToken)
	go t.pollLoop()
	return nil
}

func (t *TelegramChannel) Stop() error {
	close(t.stopCh)
	return nil
}

func (t *TelegramChannel) pollLoop() {
	log.Println("[telegram] Starting long poll loop")

	for {
		select {
		case <-t.stopCh:
			return
		default:
		}

		updates, err := t.getUpdates()
		if err != nil {
			log.Printf("[telegram] Poll error: %v", err)
			time.Sleep(3 * time.Second)
			continue
		}

		for _, upd := range updates {
			t.handleUpdate(upd)
		}

		time.Sleep(300 * time.Millisecond)
	}
}

func (t *TelegramChannel) getUpdates() ([]update, error) {
	params := url.Values{}
	params.Set("offset", strconv.Itoa(t.offset+1))
	params.Set("timeout", "10")
	params.Set("allowed_updates", `["message"]`)

	resp, err := t.http.Get(t.baseURL + "/getUpdates?" + params.Encode())
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var result struct {
		OK     bool     `json:"ok"`
		Result []update `json:"result"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, err
	}

	if !result.OK {
		return nil, fmt.Errorf("telegram API error")
	}

	for _, u := range result.Result {
		if u.UpdateID >= t.offset {
			t.offset = u.UpdateID
		}
	}

	return result.Result, nil
}

func (t *TelegramChannel) handleUpdate(upd update) {
	if upd.Message == nil || upd.Message.From.IsBot {
		return
	}

	msg := upd.Message
	userID := strconv.FormatInt(msg.From.ID, 10)
	sess := t.session.GetOrCreate("telegram", userID)

	t.bus.PublishAsync("message:"+sess.ID, bus.Message{
		Channel:   "telegram",
		UserID:    userID,
		SessionID: sess.ID,
		Content:   msg.Text,
	})

	log.Printf("[telegram] Message from %s: %.50s", msg.From.FirstName, msg.Text)
}

func (t *TelegramChannel) SendMessage(chatID int64, text string) error {
	body := sendMessageReq{
		ChatID: chatID,
		Text:   text,
	}
	data, _ := json.Marshal(body)

	resp, err := t.http.Post(
		t.baseURL+"/sendMessage",
		"application/json",
		strings.NewReader(string(data)),
	)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("telegram API error: %d", resp.StatusCode)
	}
	return nil
}
