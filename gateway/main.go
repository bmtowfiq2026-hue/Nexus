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
	"github.com/nexus/gateway/internal/channel/slack"
	"github.com/nexus/gateway/internal/channel/telegram"
	"github.com/nexus/gateway/internal/channel/webchat"
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

	registerWebChat(registry, cfg.WebChat)
	registerDiscord(registry, cfg.Discord)
	registerTelegram(registry, cfg.Telegram)
	registerSlack(registry, cfg.Slack)

	messageBus.Subscribe("agent:response", func(msg bus.Message) (bus.Response, error) {
		log.Printf("[bus] Agent response for session %s: %.80s", msg.SessionID, msg.Content)
		return bus.Response{SessionID: msg.SessionID, Content: msg.Content}, nil
	})

	mux := http.NewServeMux()

	mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"status":        "ok",
			"service":       "nexus-gateway",
			"version":       "0.5.0",
			"agent_url":     agentEndpoint,
			"channels":      fmt.Sprintf("%v", registry.List()),
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

	if ch, ok := registry.Get("webchat"); ok {
		if wc, ok := ch.(*webchat.WebChatChannel); ok {
			wc.RegisterRoutes(mux)
		}
	}

	server := &http.Server{
		Addr:    fmt.Sprintf(":%d", cfg.Port),
		Handler: corsMiddleware(mux),
	}

	go func() {
		log.Printf("Gateway starting on port %d (agent: %s)", cfg.Port, agentEndpoint)
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
	if !cfg.Enabled {
		return
	}
	r.Register("webchat", webchat.NewChannel(webchat.Config{
		Enabled: cfg.Enabled,
		Path:    cfg.Path,
	}))
}

func registerDiscord(r *channel.Registry, cfg DiscordConfig) {
	if !cfg.Enabled {
		return
	}
	r.Register("discord", discord.NewChannel(discord.Config{
		Enabled:  cfg.Enabled,
		BotToken: cfg.BotToken,
	}))
}

func registerTelegram(r *channel.Registry, cfg TelegramConfig) {
	if !cfg.Enabled {
		return
	}
	r.Register("telegram", telegram.NewChannel(telegram.Config{
		Enabled:  cfg.Enabled,
		BotToken: cfg.BotToken,
	}))
}

func registerSlack(r *channel.Registry, cfg SlackConfig) {
	if !cfg.Enabled {
		return
	}
	r.Register("slack", slack.NewChannel(slack.Config{
		Enabled:       cfg.Enabled,
		BotToken:      cfg.BotToken,
		AppToken:      cfg.AppToken,
		SigningSecret: cfg.SigningSecret,
	}))
}

type Config struct {
	Port     int            `json:"port"`
	WebChat  WebChatConfig  `json:"webchat"`
	Discord  DiscordConfig  `json:"discord"`
	Telegram TelegramConfig `json:"telegram"`
	Slack    SlackConfig    `json:"slack"`
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

func loadConfig() Config {
	cfg := Config{
		Port: 8080,
		WebChat: WebChatConfig{Enabled: true, Path: "/ws"},
		Discord:  DiscordConfig{Enabled: false},
		Telegram: TelegramConfig{Enabled: false},
		Slack:    SlackConfig{Enabled: false},
	}

	if data, err := os.ReadFile("gateway.json"); err == nil {
		var fileCfg Config
		if err := json.Unmarshal(data, &fileCfg); err == nil {
			if fileCfg.Port != 0 {
				cfg.Port = fileCfg.Port
			}
			if fileCfg.WebChat.Enabled {
				cfg.WebChat = fileCfg.WebChat
			}
			if fileCfg.Discord.Enabled {
				cfg.Discord = fileCfg.Discord
			}
			if fileCfg.Telegram.Enabled {
				cfg.Telegram = fileCfg.Telegram
			}
			if fileCfg.Slack.Enabled {
				cfg.Slack = fileCfg.Slack
			}
		}
	}

	if port := os.Getenv("NEXUS_GATEWAY_PORT"); port != "" {
		fmt.Sscanf(port, "%d", &cfg.Port)
	}

	return cfg
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
