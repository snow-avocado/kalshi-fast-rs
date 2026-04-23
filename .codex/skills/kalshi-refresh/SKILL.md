---
name: kalshi-refresh
description: Use this repo-local skill for recurring refreshes of `kalshi-fast-rs` against Kalshi's current published docs and changelog.
---
# Kalshi Refresh

Use this repo-local skill for recurring refreshes of `kalshi-fast-rs` against
Kalshi's current published docs and changelog.

## Read First

Before making changes, read:

- `AGENTS.md`
- `VERSIONING.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `README.md`
- `docs/README.md`
- `src/lib.rs`
- `docs/spec-parity.md`

## Source Of Truth

Use these upstream documents as the source of truth:

- `https://docs.kalshi.com/llms.txt`
- `https://docs.kalshi.com/changelog/rss.xml`
- `https://docs.kalshi.com/openapi.yaml`
- `https://docs.kalshi.com/asyncapi.yaml`

Re-fetch these documents during every refresh. Do not rely on cached knowledge.
Capture exact dates and spec versions from them when updating compatibility
notes.

Use this local note file only when there is repo-specific context worth
preserving:

- `docs/spec-parity.md`

## Workflow

1. Read the active `Unreleased` entry in `CHANGELOG.md`, the current crate
   version in `Cargo.toml`, and the most recent recorded compatibility tuple.
2. Fetch the current changelog RSS, OpenAPI, and AsyncAPI. Record:
   - docs snapshot date used for the refresh
   - OpenAPI version
   - AsyncAPI version
   - latest changelog date validated by the refresh
3. Identify upstream changes since the repo's last validated compatibility or
   changelog coverage, not just since the last release tag.
4. Classify each relevant upstream change as breaking or non-breaking for:
   - the upstream Kalshi contract
   - the public Rust API of this crate
5. Check the live OpenAPI and AsyncAPI for schema drift relative to the current
   code, tests, docs, and examples. Remove or update stale deprecated
   endpoints, fields, enums, examples, and tests instead of carrying them
   forward silently.
   - if a field or response shape was removed from the live schema, remove it
     from the public Rust API rather than preserving it as an optional field,
     compatibility alias, or synthesized legacy view unless `docs/spec-parity.md`
     documents an explicit exception.
6. Update `docs/spec-parity.md` only for durable human-facing notes about
   distinctions or behaviors that are not obvious from the YAML alone. Do not
   turn it into a raw diff dump.
7. Update code, tests, docs, and examples so the crate reflects current
   upstream behavior.
8. Apply the crate version bump in `Cargo.toml` and `Cargo.lock` using
   `VERSIONING.md`:
   - patch for Rust API non-breaking refreshes
   - minor for Rust API breaking changes
   - this only applies if the current branch is version agnostic.
   - always look at the current branch name. If there is a `vX.Y.Z` tag, then the version
     should be `X.Y.Z`.
   - in other words, only bump version when the branch is version agnostic.
9. Update the active `CHANGELOG.md` entry:
   - if the current branch name contains a `vX.Y.Z` tag, then update the matching entry.
   - keep the active entry in normal changelog form
   - fill the `Compatibility` block with exact upstream dates and versions
   - add a `Breaking` section when downstream Rust code must change
   - use clear bullet prefixes like `[Rust API]`, `[Upstream]`, `[Docs]`, `[CI]`, or `[Tests]` where useful
10. Update `README.md`, `docs/README.md`, and `src/lib.rs` if compatibility
    statements, examples, test guidance, or release/versioning references
    changed.
11. Run the relevant checks:
    - `cargo fmt --check`
    - `cargo test --all-targets`
    - live tests only when credentials are available and the refresh needs live
      validation:
      - `cargo test --features live-tests --test rest_public`
      - `cargo test --features live-tests --test rest_auth`
      - `cargo test --features live-tests --test ws_public`
      - `cargo test --features live-tests --test ws_auth`
12. Summarize:
   - what changed upstream
   - whether the changes are breaking
   - what compatibility tuple was validated
   - what version bump was applied or proposed
   - what code, docs, or tests were updated
   - any blockers or unresolved drift

## Expectations

- Do not leave obviously stale deprecated fields or endpoints in public structs, examples, or tests without documenting why.
- Do not preserve removed upstream schema fields by converting them to optional
  Rust fields or by synthesizing removed response shapes unless an explicit
  repo-level exception is documented.
- Do not leave `Compatibility` values as `pending` after a completed refresh
  unless an upstream source was unavailable; state the blocker explicitly.
- Prefer aligning the crate to the current Kalshi docs rather than preserving obsolete compatibility shims indefinitely.
- Keep the changelog useful for humans. Policy belongs in `VERSIONING.md`; release notes belong in `CHANGELOG.md`.
- Keep `docs/spec-parity.md` short and durable.
