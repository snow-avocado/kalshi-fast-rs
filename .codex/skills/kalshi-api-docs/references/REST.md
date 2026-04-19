# REST

> ALWAYS fetch and reference the official Kalshi documentation directly.
> NEVER rely on internal/training knowledge for Kalshi API details.

## Sources Of Truth

- `https://docs.kalshi.com/llms.txt`
- `https://docs.kalshi.com/changelog/rss.xml`
- `https://docs.kalshi.com/openapi.yaml`

Use `llms.txt` to find the human docs page you need. Use `openapi.yaml` for the current endpoint and schema contract.

## Base URLs

- Live: `https://api.elections.kalshi.com/trade-api/v2`
- Demo: `https://demo-api.kalshi.co/trade-api/v2`

## Signing

- Authenticated requests use:
  - `KALSHI-ACCESS-KEY`
  - `KALSHI-ACCESS-TIMESTAMP`
  - `KALSHI-ACCESS-SIGNATURE`
- Sign `timestamp + METHOD + path_without_query`.
- Strip query parameters before signing.
- If you pass `/portfolio/balance` to a helper, the signed path becomes `/trade-api/v2/portfolio/balance`.

## Current Authenticated Endpoints

This list is derived from the current `openapi.yaml` `security` blocks. Re-fetch before relying on it.

- Historical:
  - `GET /historical/fills`
  - `GET /historical/orders`
- Portfolio orders:
  - `GET /portfolio/orders`
  - `POST /portfolio/orders`
  - `GET /portfolio/orders/{order_id}`
  - `DELETE /portfolio/orders/{order_id}`
  - `POST /portfolio/orders/batched`
  - `DELETE /portfolio/orders/batched`
  - `POST /portfolio/orders/{order_id}/amend`
  - `POST /portfolio/orders/{order_id}/decrease`
  - `GET /portfolio/orders/queue_positions`
  - `GET /portfolio/orders/{order_id}/queue_position`
- Order groups:
  - `GET /portfolio/order_groups`
  - `POST /portfolio/order_groups/create`
  - `GET /portfolio/order_groups/{order_group_id}`
  - `DELETE /portfolio/order_groups/{order_group_id}`
  - `PUT /portfolio/order_groups/{order_group_id}/reset`
  - `PUT /portfolio/order_groups/{order_group_id}/trigger`
  - `PUT /portfolio/order_groups/{order_group_id}/limit`
- Portfolio and subaccounts:
  - `GET /portfolio/balance`
  - `POST /portfolio/subaccounts`
  - `POST /portfolio/subaccounts/transfer`
  - `GET /portfolio/subaccounts/balances`
  - `GET /portfolio/subaccounts/transfers`
  - `PUT /portfolio/subaccounts/netting`
  - `GET /portfolio/subaccounts/netting`
  - `GET /portfolio/positions`
  - `GET /portfolio/settlements`
  - `GET /portfolio/summary/total_resting_order_value`
  - `GET /portfolio/fills`
- Account and keys:
  - `GET /api_keys`
  - `POST /api_keys`
  - `POST /api_keys/generate`
  - `DELETE /api_keys/{api_key}`
  - `GET /account/limits`
- Other authenticated REST operations:
  - `GET /series/{series_ticker}/events/{ticker}/forecast_percentile_history`
  - `GET /fcm/orders`
  - `GET /fcm/positions`
  - `GET /markets/{ticker}/orderbook`
  - `GET /markets/orderbooks`
  - `GET /communications/id`
  - `GET /communications/rfqs`
  - `POST /communications/rfqs`
  - `GET /communications/rfqs/{rfq_id}`
  - `DELETE /communications/rfqs/{rfq_id}`
  - `GET /communications/quotes`
  - `POST /communications/quotes`
  - `GET /communications/quotes/{quote_id}`
  - `DELETE /communications/quotes/{quote_id}`
  - `PUT /communications/quotes/{quote_id}/accept`
  - `PUT /communications/quotes/{quote_id}/confirm`
  - `POST /multivariate_event_collections/{collection_ticker}`
  - `PUT /multivariate_event_collections/{collection_ticker}/lookup`

Endpoints not listed above are unauthenticated in the current live `openapi.yaml`.

## Error Handling

Always check error response bodies for the detailed Kalshi message. The body often contains the real reason a request failed.

## Helper Script

Use `scripts/kalshi_rest.py` for quick manual requests:

```bash
uv run .codex/skills/kalshi-api-docs/scripts/kalshi_rest.py \
  --platform demo \
  --method GET \
  --path /exchange/status
```

Authenticated example:

```bash
uv run .codex/skills/kalshi-api-docs/scripts/kalshi_rest.py \
  --platform demo \
  --method GET \
  --path /portfolio/balance \
  --api-key-id "$KALSHI_API_KEY_ID" \
  --private-key /path/to/kalshi.key
```
