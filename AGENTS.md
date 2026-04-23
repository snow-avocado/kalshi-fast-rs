# AGENTS

This repo tracks Kalshi's published docs. Use these as the upstream sources of
truth:

- `https://docs.kalshi.com/llms.txt`
- `https://docs.kalshi.com/changelog/rss.xml`
- `https://docs.kalshi.com/openapi.yaml`
- `https://docs.kalshi.com/asyncapi.yaml`

Versioning policy lives in `VERSIONING.md`.

The repo-local refresh workflow lives in `.codex/skills/kalshi-refresh/SKILL.md`.

## Local Auth Files

When working with local Kalshi credentials in this repo:

- `test_key.pem` is the demo private key. Reference `.env.test` for the matching
  demo key ID and related settings.
- `.kalshi_api_key.pem` is the live private key. Reference `.env` for the
  matching live key ID and related settings.
