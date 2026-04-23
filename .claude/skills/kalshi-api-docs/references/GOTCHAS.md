# Gotchas

## WebSocket Connections Require Auth

This applies to WebSockets.

- Even channels labeled `public` still require an authenticated WebSocket handshake.
- In WebSocket docs, `public` means public market data, not anonymous access.

## Multiple URLs Exist

- Live REST root: `https://api.elections.kalshi.com/trade-api/v2`
- Live WebSocket root: `wss://api.elections.kalshi.com/trade-api/ws/v2`
- Demo REST root: `https://demo-api.kalshi.co/trade-api/v2`
- Demo WebSocket root: `wss://demo-api.kalshi.co/trade-api/ws/v2`
- Demo credentials are separate from live credentials.
