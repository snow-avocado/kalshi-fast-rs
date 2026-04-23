---
name: kalshi-api-docs
description: Project-specific Kalshi API reference and request helpers. Use when working in this repo on Kalshi REST or WebSocket integrations and you need the current source-of-truth docs, core terminology, contract mechanics, authentication guidance, or lightweight scripts to validate assumptions.
---
# Kalshi API Docs

> ALWAYS fetch and reference the official Kalshi documentation directly.
> NEVER rely on internal/training knowledge for Kalshi API details.

## Sources Of Truth

Re-fetch these documents when you need current behavior:

- `https://docs.kalshi.com/llms.txt`
- `https://docs.kalshi.com/changelog/rss.xml`
- `https://docs.kalshi.com/openapi.yaml`
- `https://docs.kalshi.com/asyncapi.yaml`

Use `llms.txt` to discover the right docs pages before exploring further.

## Core Terminology

- `series -> event -> market` is the core hierarchy.
- A `market` is the tradeable binary contract. It resolves to `YES = $1.00` or `NO = $0.00`.
- Market tickers follow `SERIES-EVENT-MARKET`.
  Example: `KXHIGHNY-24JAN01-T60`
  Series: `KXHIGHNY`
  Event: `24JAN01`
  Market: `T60`
- Event and market lifecycle terms can change across docs revisions. Re-check `llms.txt` plus the live specs before hard-coding enums.

## Contract Model

- Kalshi contracts are binary.
- `yes_bid + no_bid = 100` cents in the standard binary framing.
- Orderbooks expose bids, not a separate ask ladder.
- Implied asks come from the opposite side:
  - `yes_ask = 100 - best_no_bid`
  - `no_ask = 100 - best_yes_bid`
- Orderbook price arrays are commonly sorted ascending, so the best bid is typically the last element. Verify against current docs before assuming this in new code.
- Pricing is migrating toward `_dollars` fields and quantities toward fixed-point fields. Re-check the live docs before preferring legacy integer-only fields.

## Detailed References

Read only the file you need:

- `references/REST.md`
- `references/WEBSOCKET.md`
- `references/GOTCHAS.md`

## Validation Helpers

Use the bundled scripts to validate assumptions quickly against demo or live endpoints:

- `scripts/kalshi_rest.py`
- `scripts/kalshi_websocket.py`

These helpers are for direct inspection and debugging, not as a replacement for the crate implementation.
