# Channel Setup Guide

Nexus connects to messaging platforms through the **Go gateway**. The gateway runs as a separate process and communicates with the Rust core via HTTP/WebSocket.

## Quick Start

```bash
# Build the gateway
cd gateway
go build -o nexus-gateway .

# Configure channels (edit gateway/gateway.json)
# Start the gateway
./nexus-gateway

# In another terminal, start the agent
nexus chat
```

The gateway listens on `http://localhost:8080`. Each channel receives messages and forwards them to the agent; responses are sent back through the same channel.

## Gateway Configuration

All 13 channels are configured in a single `gateway.json` file. Channels are **disabled by default** — set `"enabled": true` and add your credentials.

```json
{
  "port": 8080,
  "webchat":   { "enabled": true,  "path": "/ws" },
  "discord":   { "enabled": false, "bot_token": "" },
  "telegram":  { "enabled": false, "bot_token": "" },
  "slack":     { "enabled": false, "bot_token": "", "app_token": "", "signing_secret": "" },
  "matrix":    { "enabled": false, "homeserver": "", "access_token": "", "room_id": "" },
  "whatsapp":  { "enabled": false, "webhook_path": "/webhook/whatsapp", "verify_token": "", "api_token": "" },
  "signal":    { "enabled": false, "phone_number": "", "signal_api": "http://localhost:8080" },
  "irc":       { "enabled": false, "server": "", "port": 6667, "nick": "nexus-bot", "channel": "", "password": "" },
  "googlechat": { "enabled": false, "webhook_url": "", "space_id": "" },
  "msteams":   { "enabled": false, "webhook_url": "", "app_id": "", "app_secret": "" },
  "line":      { "enabled": false, "channel_secret": "", "channel_token": "" },
  "messenger": { "enabled": false, "page_id": "", "access_token": "", "verify_token": "" },
  "twilio":    { "enabled": false, "account_sid": "", "auth_token": "", "from_number": "" }
}
```

## WebChat

The WebChat channel provides a browser-based chat interface served directly by the gateway.

**Type:** WebSocket + REST  
**Implementation:** Embedded HTML/JS/CSS (`internal/channel/webchat/index.html`)

### Setup
```json
"webchat": { "enabled": true, "path": "/ws" }
```

Open `http://localhost:8080/` in your browser. The chat UI connects to the WebSocket at `/ws` automatically.

### REST API
```bash
curl -X POST http://localhost:8080/api/message \
  -H "Content-Type: application/json" \
  -d '{"session_id": "user-123", "content": "Hello!"}'

curl http://localhost:8080/health
curl http://localhost:8080/channels
```

---

## Discord

**Type:** Gateway API (bot)  
**Implementation:** Discord Bot API via WebSocket gateway

### Prerequisites
1. Create a bot at https://discord.com/developers/applications
2. Go to **Bot** settings → enable **Message Content Intent**
3. Copy the bot token
4. Invite bot to your server using the OAuth2 URL generator with `bot` + `messages.read` scopes

### Configuration
```json
"discord": { "enabled": true, "bot_token": "YOUR_DISCORD_BOT_TOKEN" }
```

### Notes
- DMs from unknown users are rejected by default
- Works in any channel the bot has access to

---

## Telegram

**Type:** Bot API (polling)  
**Implementation:** Telegram Bot API via long-polling

### Prerequisites
1. Open Telegram and search for [@BotFather](https://t.me/botfather)
2. Send `/newbot` and follow the prompts
3. Copy the bot token you receive

### Configuration
```json
"telegram": { "enabled": true, "bot_token": "YOUR_TELEGRAM_BOT_TOKEN" }
```

### Commands
- `/start` — Begin interaction
- `/help` — Show available commands
- Any text message is processed by the agent

---

## Slack

**Type:** Events API (Socket Mode)  
**Implementation:** Slack Events API via Socket Mode

### Prerequisites
1. Create an app at https://api.slack.com/apps
2. Enable **Socket Mode**
3. Add these Bot Token Scopes: `chat:write`, `im:history`, `im:read`, `channels:history`
4. Install the app to your workspace
5. Copy the **Bot Token** (`xoxb-...`) and **App Token** (`xapp-...`)

### Configuration
```json
"slack": {
  "enabled": true,
  "bot_token": "xoxb-YOUR-BOT-TOKEN",
  "app_token": "xapp-YOUR-APP-TOKEN",
  "signing_secret": "YOUR-SIGNING-SECRET"
}
```

### Events handled
- `message.im` — Direct messages to the bot
- `message.channel` — Messages in channels where the bot is added

---

## Matrix

**Type:** Client-Server API (polling)  
**Implementation:** Syncs via `GET /_matrix/client/v3/sync` using access token

### Prerequisites
1. Register an account on any Matrix homeserver (e.g., matrix.org)
2. Get an access token:
   ```bash
   curl -X POST "https://YOUR_HOMESERVER/_matrix/client/v3/login" \
     -H "Content-Type: application/json" \
     -d '{"type":"m.login.password","user":"YOUR_USERNAME","password":"YOUR_PASSWORD"}'
   ```
   Copy the `access_token` from the response.
3. Find your room ID — enable developer mode in Element, then go to Room Settings → Advanced → Room ID.

### Configuration
```json
"matrix": {
  "enabled": true,
  "homeserver": "https://matrix.org",
  "access_token": "syt_...",
  "room_id": "!yourroomid:matrix.org"
}
```

### Notes
- Polls every 500ms for new events
- Supports both encrypted and unencrypted rooms (your access token handles auth)

---

## IRC

**Type:** Native TCP protocol  
**Implementation:** Raw IRC protocol via `net.Conn` (NICK, USER, JOIN, PRIVMSG)

### Prerequisites
1. Choose an IRC network (e.g., Libera Chat, OFTC, or your own)
2. Register a nickname if required by the network

### Configuration
```json
"irc": {
  "enabled": true,
  "server": "irc.libera.chat",
  "port": 6667,
  "nick": "nexus-bot",
  "channel": "#my-channel",
  "password": ""
}
```

- Leave `password` empty if the server doesn't require one
- Use port `6697` with TLS if supported by the server (future enhancement)
- The bot joins the specified `channel` and responds to messages

---

## WhatsApp

**Type:** Webhook (Cloud API)  
**Implementation:** WhatsApp Cloud API via incoming webhook

### Prerequisites
1. Go to https://developers.facebook.com → WhatsApp → Get Started
2. Create a Meta Business Account and WhatsApp Business Account
3. Set up a webhook endpoint URL (must be HTTPS — use ngrok for local testing):
   ```bash
   ngrok http 8080
   # Your public URL: https://abc123.ngrok.io
   ```
4. Configure the webhook URL in Meta Developer Dashboard pointing to `https://your-ngrok.ngrok.io/webhook/whatsapp`
5. Copy the **Verify Token** (you choose this) and **API Token** (permanent or temporary)

### Configuration
```json
"whatsapp": {
  "enabled": true,
  "webhook_path": "/webhook/whatsapp",
  "verify_token": "YOUR_VERIFY_TOKEN",
  "api_token": "YOUR_API_TOKEN"
}
```

### Notes
- The gateway automatically handles the verification challenge
- Works with both personal and business phone numbers (must be approved by Meta)

---

## Signal

**Type:** Webhook (polling, via signal-cli REST API)  
**Implementation:** Polls signal-cli REST API receive endpoint

### Prerequisites
1. Install [signal-cli](https://github.com/AsamK/signal-cli) on your server:
   ```bash
   # Linux
   sudo apt install signal-cli

   # Or via Docker
   docker run -d --name signal-cli -p 8080:8080 signal-cli-rest-api
   ```
2. Register your phone number with signal-cli:
   ```bash
   signal-cli -u +1234567890 register
   signal-cli -u +1234567890 verify CODE_FROM_SMS
   ```
3. Start the signal-cli REST API on port `8080`:
   ```bash
   signal-cli -u +1234567890 daemon --http 0.0.0.0:8080
   ```

### Configuration
```json
"signal": {
  "enabled": true,
  "phone_number": "+1234567890",
  "signal_api": "http://localhost:8080"
}
```

### Notes
- The gateway polls `GET {signal_api}/v1/receive/{phone_number}` every second
- signal-cli must be running and reachable from the gateway

---

## Google Chat

**Type:** Incoming webhook  
**Implementation:** Google Chat webhooks — simplest setup, no polling

### Prerequisites
1. Open Google Chat
2. Go to a space → **Apps & Integrations** → **Manage webhooks**
3. Click **Add webhook**, give it a name, and copy the webhook URL

### Configuration
```json
"googlechat": {
  "enabled": true,
  "webhook_url": "https://chat.googleapis.com/v1/spaces/.../messages?key=...",
  "space_id": ""
}
```

### Notes
- Google Chat webhooks are **incoming only** out of the box
- `space_id` is reserved for future bidirectional support
- Only use where one-way notifications from Nexus to Google Chat are sufficient

---

## Microsoft Teams

**Type:** Incoming webhook + OAuth  
**Implementation:** Teams webhook with App ID/Secret for message sending

### Prerequisites
1. Go to https://dev.teams.microsoft.com/ — create a new app or use an existing one
2. Generate an **Incoming Webhook** in your Teams channel:
   - Channel → **...** → **Connectors** → **Incoming Webhook** → Configure
   - Copy the webhook URL
3. Register the app in Azure AD to get `app_id` and `app_secret` (for sending messages)

### Configuration
```json
"msteams": {
  "enabled": true,
  "webhook_url": "https://your-tenant.webhook.office.com/webhookb2/...",
  "app_id": "YOUR_AZURE_APP_ID",
  "app_secret": "YOUR_AZURE_APP_SECRET"
}
```

### Notes
- `webhook_url` handles incoming messages
- `app_id` + `app_secret` authenticate outgoing messages via Microsoft Graph API

---

## LINE

**Type:** Webhook (LINE Messaging API)  
**Implementation:** LINE Messaging API webhook

### Prerequisites
1. Go to https://developers.line.biz/ → create a new **Provider** → **Channel** (Messaging API)
2. In the channel settings, enable **Webhook** and set the URL to `https://your-server/webhook/line`
3. Copy the **Channel Secret** from Basic Settings
4. Issue a **Channel Access Token** from Messaging API tab

### Configuration
```json
"line": {
  "enabled": true,
  "channel_secret": "YOUR_CHANNEL_SECRET",
  "channel_token": "YOUR_CHANNEL_ACCESS_TOKEN"
}
```

### Notes
- LINE requires HTTPS for webhooks (use ngrok for local testing)
- Signature verification is handled automatically using `channel_secret`

---

## Messenger (Facebook)

**Type:** Webhook (Facebook Messenger Platform)  
**Implementation:** Messenger Platform API webhook

### Prerequisites
1. Go to https://developers.facebook.com/ — create a Facebook App
2. Add the **Messenger** product to your app
3. Generate a **Page Access Token** (link your Facebook Page)
4. Set up the webhook URL to `https://your-server/webhook/messenger`
5. Choose a **Verify Token** of your choice
6. Subscribe to `messages` and `messaging_postbacks` events

### Configuration
```json
"messenger": {
  "enabled": true,
  "page_id": "YOUR_FACEBOOK_PAGE_ID",
  "access_token": "EAAx...YOUR_PAGE_ACCESS_TOKEN",
  "verify_token": "YOUR_VERIFY_TOKEN"
}
```

### Notes
- Requires HTTPS (use ngrok for local dev)
- The verify token can be any string — it only needs to match between your config and Meta Dashboard

---

## Twilio (SMS)

**Type:** Webhook + custom routes  
**Implementation:** Twilio SMS API with webhook for inbound, REST for outbound

### Prerequisites
1. Sign up at https://www.twilio.com — get a phone number capable of SMS
2. Copy your **Account SID** and **Auth Token** from the Twilio Console
3. In your Twilio phone number config, set the webhook URL for incoming messages:
   - Phone Numbers → Manage → Active Numbers → your number
   - Set "A message comes in" to `https://your-server/webhook/twilio`
   - Method: `POST`

### Configuration
```json
"twilio": {
  "enabled": true,
  "account_sid": "ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
  "auth_token": "YOUR_AUTH_TOKEN",
  "from_number": "+1234567890"
}
```

### Notes
- Uses Basic Auth with `account_sid` and `auth_token` for Twilio API calls
- `from_number` must be your Twilio phone number in E.164 format
- Handles both incoming SMS webhooks and outgoing message sends

---

## Exposing Webhooks Publicly (Local Development)

Nearly all webhook-based channels (WhatsApp, LINE, Messenger, Twilio) require an HTTPS endpoint. For local development, use **ngrok**:

```bash
# Install ngrok from https://ngrok.com
ngrok http 8080
# Output: https://abc123.ngrok.io -> http://localhost:8080
```

Then configure each platform's webhook URL as `https://abc123.ngrok.io/webhook/<channel>`.

## Architecture & Message Flow

```
Channel receives message
        │
        ▼
Channel.Send(msg) ──► bus.Bus.Publish(msg)
                            │
                            ▼
                    Agent processes message
                            │
                            ▼
                    Agent.Send(response) ──► bus.Bus.Publish(response)
                            │
                            ▼
                    Channel receives response on bus
                            │
                            ▼
                    Channel sends response to platform
```

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

For webhook-based channels, use the `internal/channel/webhook` base package:

```go
import "github.com/nexus/gateway/internal/channel/webhook"

type Channel struct {
    *webhook.Channel  // Embeds all webhook logic
    config Config
}
```

See `gateway/internal/channel/webhook/webhook.go` for the base implementation and any existing channel for an example.
