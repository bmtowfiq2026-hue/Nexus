package irc

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"strings"
	"time"

	"github.com/nexus/gateway/internal/bus"
)

type Config struct {
	Enabled  bool   `json:"enabled"`
	Server   string `json:"server"`
	Port     int    `json:"port"`
	Nick     string `json:"nick"`
	Channel  string `json:"channel"`
	Password string `json:"password"`
}

type Channel struct {
	config Config
	bus    *bus.Bus
	conn   net.Conn
	stopCh chan struct{}
}

func NewChannel(cfg Config) *Channel {
	if cfg.Port == 0 { cfg.Port = 6667 }
	return &Channel{config: cfg, stopCh: make(chan struct{})}
}

func (c *Channel) Name() string { return "irc" }

func (c *Channel) Start(b *bus.Bus) error {
	if !c.config.Enabled { log.Println("[irc] Disabled"); return nil }
	c.bus = b
	go c.connectLoop()
	log.Println("[irc] Ready")
	return nil
}

func (c *Channel) Stop() error { close(c.stopCh); return nil }

func (c *Channel) connectLoop() {
	for {
		select {
		case <-c.stopCh: return
		default:
			err := c.connect()
			if err != nil {
				log.Printf("[irc] Connection error: %v, retrying in 30s", err)
				time.Sleep(30 * time.Second)
			}
		}
	}
}

func (c *Channel) connect() error {
	var err error
	addr := c.config.Server
	if c.config.Port > 0 { addr = net.JoinHostPort(c.config.Server, fmt.Sprintf("%d", c.config.Port)) }
	c.conn, err = net.DialTimeout("tcp", addr, 10*time.Second)
	if err != nil { return err }
	defer c.conn.Close()

	fmt.Fprintf(c.conn, "NICK %s\r\n", c.config.Nick)
	fmt.Fprintf(c.conn, "USER %s 0 * :Nexus Bot\r\n", c.config.Nick)
	time.Sleep(1 * time.Second)

	if c.config.Password != "" {
		fmt.Fprintf(c.conn, "PASS %s\r\n", c.config.Password)
	}
	fmt.Fprintf(c.conn, "JOIN %s\r\n", c.config.Channel)

	scanner := bufio.NewScanner(c.conn)
	for scanner.Scan() {
		select {
		case <-c.stopCh: return nil
		default:
		}
		line := scanner.Text()
		log.Printf("[irc] %s", line)

		if strings.HasPrefix(line, "PING") {
			fmt.Fprintf(c.conn, "PONG %s\r\n", line[5:])
			continue
		}

		parts := strings.SplitN(line, " ", 4)
		if len(parts) >= 4 && parts[1] == "PRIVMSG" {
			msg := strings.TrimPrefix(parts[3], ":")
			user := strings.SplitN(parts[0], "!", 2)[0][1:]
			c.bus.PublishAsync("message:irc:"+user, bus.Message{
				Channel: "irc", UserID: user, SessionID: "irc:" + user, Content: msg,
			})
		}
	}
	return scanner.Err()
}
