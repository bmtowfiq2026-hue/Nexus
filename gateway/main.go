package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/nexus/gateway/internal/bus"
	"github.com/nexus/gateway/internal/channel"
	"github.com/nexus/gateway/internal/channel/discord"
	"github.com/nexus/gateway/internal/channel/googlechat"
	"github.com/nexus/gateway/internal/channel/irc"
	"github.com/nexus/gateway/internal/channel/line"
	"github.com/nexus/gateway/internal/channel/matrix"
	"github.com/nexus/gateway/internal/channel/messenger"
	"github.com/nexus/gateway/internal/channel/msteams"
	sigchannel "github.com/nexus/gateway/internal/channel/signal"
	"github.com/nexus/gateway/internal/channel/slack"
	"github.com/nexus/gateway/internal/channel/telegram"
	"github.com/nexus/gateway/internal/channel/twilio"
	"github.com/nexus/gateway/internal/channel/webchat"
	"github.com/nexus/gateway/internal/channel/whatsapp"
)

var agentEndpoint string

func main() {
	log.SetPrefix("[nexus-gateway] ")
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)

	agentEndpoint = os.Getenv("NEXUS_AGENT_ENDPOINT")
	if agentEndpoint == "" {
		agentEndpoint = "http://localhost:9876"
	}

	cfg := loadConfig()
	messageBus := bus.New()
	registry := channel.NewRegistry()

	// Register all channels
	registerWebChat(registry, cfg.WebChat)
	registerDiscord(registry, cfg.Discord)
	registerTelegram(registry, cfg.Telegram)
	registerSlack(registry, cfg.Slack)
	registerMatrix(registry, cfg.Matrix)
	registerWhatsApp(registry, cfg.WhatsApp)
	registerSignal(registry, cfg.Signal)
	registerIRC(registry, cfg.IRC)
	registerGoogleChat(registry, cfg.GoogleChat)
	registerMSTeams(registry, cfg.MSTeams)
	registerLINE(registry, cfg.LINE)
	registerMessenger(registry, cfg.Messenger)
	registerTwilio(registry, cfg.Twilio)

	messageBus.Subscribe("agent:response", func(msg bus.Message) (bus.Response, error) {
		log.Printf("[bus] Agent response for session %s: %.80s", msg.SessionID, msg.Content)
		return bus.Response{SessionID: msg.SessionID, Content: msg.Content}, nil
	})

	mux := http.NewServeMux()

	// API routes
	mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"status":   "ok",
			"service":  "nexus-gateway",
			"version":  "0.5.0",
			"agent_url": agentEndpoint,
			"channels": registry.List(),
		})
	})

	mux.HandleFunc("/channels", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(registry.List())
	})

	mux.HandleFunc("/api/message", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != "POST" {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		var msg struct {
			SessionID string `json:"session_id"`
			Content   string `json:"content"`
			Channel   string `json:"channel"`
		}
		if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
			http.Error(w, "Invalid JSON", http.StatusBadRequest)
			return
		}
		if msg.Content == "" {
			http.Error(w, "Content required", http.StatusBadRequest)
			return
		}
		if msg.SessionID == "" {
			msg.SessionID = fmt.Sprintf("gateway-%s", channel.NewRegistry())
		}
		agentResp, err := forwardToAgent(msg.SessionID, msg.Content)
		if err != nil {
			log.Printf("Agent error: %v", err)
			http.Error(w, fmt.Sprintf("Agent error: %v", err), http.StatusBadGateway)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"session_id": msg.SessionID,
			"response":   agentResp,
		})
	})

	// Register webchat routes (serves UI at /, WS at /ws, API at /api/chat)
	if ch, ok := registry.Get("webchat"); ok {
		if wc, ok := ch.(*webchat.WebChatChannel); ok {
			wc.RegisterRoutes(mux)
		}
	}

	// Register webhook routes for channels that support them
	registerChannelWebhooks(registry, mux)

	server := &http.Server{
		Addr:    fmt.Sprintf(":%d", cfg.Port),
		Handler: corsMiddleware(mux),
	}

	go func() {
		log.Printf("Gateway starting on port %d (agent: %s)", cfg.Port, agentEndpoint)
		log.Printf("WebChat UI: http://localhost:%d/", cfg.Port)
		for _, name := range registry.List() {
			log.Printf("  Channel active: %s", name)
		}
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server error: %v", err)
		}
	}()

	if err := registry.StartAll(messageBus); err != nil {
		log.Fatalf("Failed to start channels: %v", err)
	}

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	log.Println("Shutting down gateway...")
	registry.StopAll()
	server.Close()
}

func registerChannelWebhooks(registry *channel.Registry, mux *http.ServeMux) {
	for _, name := range registry.List() {
		ch, _ := registry.Get(name)
		if wh, ok := ch.(webhookRegistrar); ok {
			wh.RegisterRoutes(mux)
			log.Printf("  Webhook registered: %s", name)
		}
	}
}

type webhookRegistrar interface {
	RegisterRoutes(mux *http.ServeMux)
}

func forwardToAgent(sessionID, content string) (string, error) {
	body := map[string]string{
		"session_id": sessionID,
		"message":    content,
	}
	jsonBody, err := json.Marshal(body)
	if err != nil {
		return "", fmt.Errorf("marshal error: %w", err)
	}

	resp, err := http.Post(
		fmt.Sprintf("%s/api/agent", agentEndpoint),
		"application/json",
		bytes.NewReader(jsonBody),
	)
	if err != nil {
		return "", fmt.Errorf("connection to agent failed: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("read response error: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("agent returned status %d: %s", resp.StatusCode, string(respBody))
	}

	var result struct {
		Response string `json:"response"`
	}
	if err := json.Unmarshal(respBody, &result); err != nil {
		return string(respBody), nil
	}

	return result.Response, nil
}

func registerWebChat(r *channel.Registry, cfg WebChatConfig) {
	if !cfg.Enabled { return }
	r.Register("webchat", webchat.NewChannel(webchat.Config{
		Enabled: cfg.Enabled, Path: cfg.Path,
	}))
}

func registerDiscord(r *channel.Registry, cfg DiscordConfig) {
	if !cfg.Enabled { return }
	r.Register("discord", discord.NewChannel(discord.Config{
		Enabled: cfg.Enabled, BotToken: cfg.BotToken,
	}))
}

func registerTelegram(r *channel.Registry, cfg TelegramConfig) {
	if !cfg.Enabled { return }
	r.Register("telegram", telegram.NewChannel(telegram.Config{
		Enabled: cfg.Enabled, BotToken: cfg.BotToken,
	}))
}

func registerSlack(r *channel.Registry, cfg SlackConfig) {
	if !cfg.Enabled { return }
	r.Register("slack", slack.NewChannel(slack.Config{
		Enabled: cfg.Enabled, BotToken: cfg.BotToken,
		AppToken: cfg.AppToken, SigningSecret: cfg.SigningSecret,
	}))
}

func registerMatrix(r *channel.Registry, cfg MatrixConfig) {
	if !cfg.Enabled { return }
	r.Register("matrix", matrix.NewChannel(matrix.Config{
		Enabled: cfg.Enabled, Homeserver: cfg.Homeserver,
		AccessToken: cfg.AccessToken, RoomID: cfg.RoomID,
	}))
}

func registerWhatsApp(r *channel.Registry, cfg WhatsAppConfig) {
	if !cfg.Enabled { return }
	r.Register("whatsapp", whatsapp.NewChannel(whatsapp.Config{
		Enabled: cfg.Enabled, WebhookPath: cfg.WebhookPath,
		VerifyToken: cfg.VerifyToken, APIToken: cfg.APIToken,
	}))
}

func registerSignal(r *channel.Registry, cfg SignalConfig) {
	if !cfg.Enabled { return }
	r.Register("signal", sigchannel.NewChannel(sigchannel.Config{
		Enabled: cfg.Enabled, PhoneNumber: cfg.PhoneNumber, SignalAPI: cfg.SignalAPI,
	}))
}

func registerIRC(r *channel.Registry, cfg IRCConfig) {
	if !cfg.Enabled { return }
	r.Register("irc", irc.NewChannel(irc.Config{
		Enabled: cfg.Enabled, Server: cfg.Server, Port: cfg.Port,
		Nick: cfg.Nick, Channel: cfg.Channel, Password: cfg.Password,
	}))
}

func registerGoogleChat(r *channel.Registry, cfg GoogleChatConfig) {
	if !cfg.Enabled { return }
	r.Register("googlechat", googlechat.NewChannel(googlechat.Config{
		Enabled: cfg.Enabled, WebhookURL: cfg.WebhookURL, SpaceID: cfg.SpaceID,
	}))
}

func registerMSTeams(r *channel.Registry, cfg MSTeamsConfig) {
	if !cfg.Enabled { return }
	r.Register("msteams", msteams.NewChannel(msteams.Config{
		Enabled: cfg.Enabled, WebhookURL: cfg.WebhookURL,
		AppID: cfg.AppID, AppSecret: cfg.AppSecret,
	}))
}

func registerLINE(r *channel.Registry, cfg LINEConfig) {
	if !cfg.Enabled { return }
	r.Register("line", line.NewChannel(line.Config{
		Enabled: cfg.Enabled, ChannelSecret: cfg.ChannelSecret, ChannelToken: cfg.ChannelToken,
	}))
}

func registerMessenger(r *channel.Registry, cfg MessengerConfig) {
	if !cfg.Enabled { return }
	r.Register("messenger", messenger.NewChannel(messenger.Config{
		Enabled: cfg.Enabled, PageID: cfg.PageID,
		AccessToken: cfg.AccessToken, VerifyToken: cfg.VerifyToken,
	}))
}

func registerTwilio(r *channel.Registry, cfg TwilioConfig) {
	if !cfg.Enabled { return }
	r.Register("twilio", twilio.NewChannel(twilio.Config{
		Enabled: cfg.Enabled, AccountSID: cfg.AccountSID,
		AuthToken: cfg.AuthToken, FromNumber: cfg.FromNumber,
	}))
}

type Config struct {
	Port      int              `json:"port"`
	WebChat   WebChatConfig    `json:"webchat"`
	Discord   DiscordConfig    `json:"discord"`
	Telegram  TelegramConfig   `json:"telegram"`
	Slack     SlackConfig      `json:"slack"`
	Matrix    MatrixConfig     `json:"matrix"`
	WhatsApp  WhatsAppConfig   `json:"whatsapp"`
	Signal    SignalConfig     `json:"signal"`
	IRC       IRCConfig        `json:"irc"`
	GoogleChat GoogleChatConfig `json:"googlechat"`
	MSTeams   MSTeamsConfig    `json:"msteams"`
	LINE      LINEConfig       `json:"line"`
	Messenger MessengerConfig  `json:"messenger"`
	Twilio    TwilioConfig     `json:"twilio"`
}

type WebChatConfig struct {
	Enabled bool   `json:"enabled"`
	Path    string `json:"path"`
}
type DiscordConfig struct {
	Enabled  bool   `json:"enabled"`
	BotToken string `json:"bot_token"`
}
type TelegramConfig struct {
	Enabled  bool   `json:"enabled"`
	BotToken string `json:"bot_token"`
}
type SlackConfig struct {
	Enabled       bool   `json:"enabled"`
	BotToken      string `json:"bot_token"`
	AppToken      string `json:"app_token"`
	SigningSecret string `json:"signing_secret"`
}
type MatrixConfig struct {
	Enabled     bool   `json:"enabled"`
	Homeserver  string `json:"homeserver"`
	AccessToken string `json:"access_token"`
	RoomID      string `json:"room_id"`
}
type WhatsAppConfig struct {
	Enabled     bool   `json:"enabled"`
	WebhookPath string `json:"webhook_path"`
	VerifyToken string `json:"verify_token"`
	APIToken    string `json:"api_token"`
}
type SignalConfig struct {
	Enabled     bool   `json:"enabled"`
	PhoneNumber string `json:"phone_number"`
	SignalAPI   string `json:"signal_api"`
}
type IRCConfig struct {
	Enabled  bool   `json:"enabled"`
	Server   string `json:"server"`
	Port     int    `json:"port"`
	Nick     string `json:"nick"`
	Channel  string `json:"channel"`
	Password string `json:"password"`
}
type GoogleChatConfig struct {
	Enabled    bool   `json:"enabled"`
	WebhookURL string `json:"webhook_url"`
	SpaceID    string `json:"space_id"`
}
type MSTeamsConfig struct {
	Enabled    bool   `json:"enabled"`
	WebhookURL string `json:"webhook_url"`
	AppID      string `json:"app_id"`
	AppSecret  string `json:"app_secret"`
}
type LINEConfig struct {
	Enabled       bool   `json:"enabled"`
	ChannelSecret string `json:"channel_secret"`
	ChannelToken  string `json:"channel_token"`
}
type MessengerConfig struct {
	Enabled     bool   `json:"enabled"`
	PageID      string `json:"page_id"`
	AccessToken string `json:"access_token"`
	VerifyToken string `json:"verify_token"`
}
type TwilioConfig struct {
	Enabled    bool   `json:"enabled"`
	AccountSID string `json:"account_sid"`
	AuthToken  string `json:"auth_token"`
	FromNumber string `json:"from_number"`
}

func loadConfig() Config {
	cfg := Config{
		Port: 8080,
		WebChat: WebChatConfig{Enabled: true, Path: "/ws"},
		Discord:  DiscordConfig{Enabled: false},
		Telegram: TelegramConfig{Enabled: false},
		Slack:    SlackConfig{Enabled: false},
		Matrix:   MatrixConfig{Enabled: false},
		WhatsApp: WhatsAppConfig{Enabled: false},
		Signal:   SignalConfig{Enabled: false},
		IRC:      IRCConfig{Enabled: false, Port: 6667},
		GoogleChat: GoogleChatConfig{Enabled: false},
		MSTeams: MSTeamsConfig{Enabled: false},
		LINE:    LINEConfig{Enabled: false},
		Messenger: MessengerConfig{Enabled: false},
		Twilio:    TwilioConfig{Enabled: false},
	}

	if data, err := os.ReadFile("gateway.json"); err == nil {
		var fileCfg Config
		if err := json.Unmarshal(data, &fileCfg); err == nil {
			if fileCfg.Port != 0 { cfg.Port = fileCfg.Port }
			mergeConfig(&cfg.WebChat, &fileCfg.WebChat)
			mergeConfig(&cfg.Discord, &fileCfg.Discord)
			mergeConfig(&cfg.Telegram, &fileCfg.Telegram)
			mergeConfig(&cfg.Slack, &fileCfg.Slack)
			mergeConfig(&cfg.Matrix, &fileCfg.Matrix)
			mergeConfig(&cfg.WhatsApp, &fileCfg.WhatsApp)
			mergeConfig(&cfg.Signal, &fileCfg.Signal)
			mergeConfig(&cfg.IRC, &fileCfg.IRC)
			mergeConfig(&cfg.GoogleChat, &fileCfg.GoogleChat)
			mergeConfig(&cfg.MSTeams, &fileCfg.MSTeams)
			mergeConfig(&cfg.LINE, &fileCfg.LINE)
			mergeConfig(&cfg.Messenger, &fileCfg.Messenger)
			mergeConfig(&cfg.Twilio, &fileCfg.Twilio)
		}
	}

	if port := os.Getenv("NEXUS_GATEWAY_PORT"); port != "" {
		fmt.Sscanf(port, "%d", &cfg.Port)
	}

	return cfg
}

func mergeConfig[T any](dst *T, src *T) {
	if src == nil { return }
	*dst = *src
}

func corsMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")
		if r.Method == "OPTIONS" {
			w.WriteHeader(http.StatusOK)
			return
		}
		next.ServeHTTP(w, r)
	})
}
