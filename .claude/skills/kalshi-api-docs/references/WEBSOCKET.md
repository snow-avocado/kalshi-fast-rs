# WebSocket

> ALWAYS fetch and reference the official Kalshi documentation directly.
> NEVER rely on internal/training knowledge for Kalshi API details.

## Sources Of Truth

- `https://docs.kalshi.com/llms.txt`
- `https://docs.kalshi.com/changelog/rss.xml`
- `https://docs.kalshi.com/asyncapi.yaml`

Use `asyncapi.yaml` as the contract for channels, messages, and connection semantics.

## URLs

- Live: `wss://api.elections.kalshi.com/trade-api/ws/v2`
- Demo: `wss://demo-api.kalshi.co/trade-api/ws/v2`

## Authentication

- All WebSocket connections require authentication.
- The `public` label in WebSocket docs means the channel carries public information.
- It does not mean the connection can be opened anonymously.
- Sign the handshake as `timestamp + GET + /trade-api/ws/v2`.

## Helper Script

Use `scripts/kalshi_websocket.py` for quick validation. `--timeout` is required so long-running subscriptions end naturally.

```bash
uv run .codex/skills/kalshi-api-docs/scripts/kalshi_websocket.py \
  --platform demo \
  --timeout 15 \
  --api-key-id "$KALSHI_API_KEY_ID" \
  --private-key /path/to/kalshi.key \
  --channel ticker
```

For anything beyond a basic subscription flow, re-check `asyncapi.yaml` first.
