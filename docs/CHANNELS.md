# Channel Setup Guide

Nexus connects to messaging platforms through the **Go gateway**. The gateway runs as a separate process and communicates with the Rust core via HTTP/WebSocket.

## Quick Start

```bash
# Build the gateway
cd gateway
go build -o nexus-gateway .

# Configure channels
# Edit gateway/gateway.json with your bot tokens

# Start the gateway
./nexus-gateway

# In another terminal, start the agent
nexus chat
```

## Gateway Configuration

```json
{
  "port": 8080,
  "webchat": {
    "enabled": true,
    "path": "/ws"
  },
  "discord": {
    "enabled": true,
    "bot_token": "YOUR_DISCORD_BOT_TOKEN"
  },
  "telegram": {
    "enabled": true,
    "bot_token": "YOUR_TELEGRAM_BOT_TOKEN"
  },
  "slack": {
    "enabled": true,
    "bot_token": "xoxb-YOUR-SLACK-BOT-TOKEN",
    "app_token": "xapp-YOUR-SLACK-APP-TOKEN",
    "signing_secret": "YOUR_SIGNING_SECRET"
  }
}
```

## WebChat

The WebChat channel provides a browser-based chat interface:

```bash
# Enable in gateway.json
"webchat": { "enabled": true, "path": "/ws" }

# Open in browser
open http://localhost:8080/chat
```

### WebSocket API

Connect programmatically:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');
ws.onopen = () => ws.send(JSON.stringify({
  type: 'message',
  session_id: 'user-123',
  content: 'Hello Nexus!'
}));
ws.onmessage = (event) => {
  const reply = JSON.parse(event.data);
  console.log('Nexus:', reply.content);
};
```

### REST API

```bash
# Send a message
curl -X POST http://localhost:8080/api/message \
  -H "Content-Type: application/json" \
  -d '{"session_id": "user-123", "content": "Hello!"}'

# Health check
curl http://localhost:8080/health

# List active channels
curl http://localhost:8080/channels
```

## Discord

### Prerequisites
1. Create a bot at https://discord.com/developers/applications
2. Enable **Message Content Intent** in the Bot settings
3. Invite bot to server with `bot` and `messages.read` scopes

### Configuration
```json
{
  "discord": {
    "enabled": true,
    "bot_token": "YOUR_DISCORD_BOT_TOKEN"
  }
}
```

### Security
- DMs from unknown users are rejected by default
- Use Discord's role-based permissions for channel access

## Telegram

### Prerequisites
1. Create a bot via [@BotFather](https://t.me/botfather) on Telegram
2. Get your bot token

### Configuration
```json
{
  "telegram": {
    "enabled": true,
    "bot_token": "YOUR_TELEGRAM_BOT_TOKEN"
  }
}
```

### Commands
- `/start` — Begin interaction
- `/help` — Show available commands
- Any text message is processed by the agent

## Slack

### Prerequisites
1. Create an app at https://api.slack.com/apps
2. Enable **Socket Mode**
3. Add bot token scopes: `chat:write`, `im:history`, `im:read`
4. Install app to workspace

### Configuration
```json
{
  "slack": {
    "enabled": true,
    "bot_token": "xoxb-YOUR-BOT-TOKEN",
    "app_token": "xapp-YOUR-APP-TOKEN",
    "signing_secret": "YOUR-SIGNING-SECRET"
  }
}
```

### Events
- `message.im` — Direct messages
- `message.channel` — Messages in channels where bot is added

## Adding a New Channel

To add a custom channel:

1. Create `gateway/internal/channel/<name>/<name>.go`
2. Implement the `Channel` interface
3. Register in `gateway/main.go`

```go
type Channel interface {
    Name() string
    Start(bus *bus.Bus) error
    Stop() error
}
```

See existing channels (discord, telegram, slack, webchat) for implementation examples.
