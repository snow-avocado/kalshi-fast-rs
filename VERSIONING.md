# Versioning Policy

`kalshi-fast-rs` tracks a fast-moving upstream Kalshi API. One version number is
not enough to communicate both public Rust API compatibility and the exact
upstream docs snapshot that the crate currently follows, so this repository uses
separate signals for those two concerns.

## What Each Artifact Means

- Cargo crate version: compatibility of the public Rust API exposed by this repository
- Compatibility tuple: the upstream Kalshi docs snapshot and spec versions tracked by a release
- `CHANGELOG.md`: blended release history covering Rust API changes, upstream contract movement, docs/parity updates, and operational changes

## Crate Version Scope

The crate keeps a `major.minor.patch` version, but that version is scoped to the
public Rust API of this repository. It does not, by itself, fully describe the
state of upstream Kalshi compatibility.

While the crate remains below `1.0.0`:

- Patch releases are for Rust API non-breaking changes, internal fixes, tests, docs, CI changes, and upstream additive alignment that should not require downstream code changes.
- Minor releases are for any intentional breaking change to the public Rust API, or any change likely to require downstream code changes even if it originates from upstream Kalshi churn.
- Major releases are reserved for a future stable `1.0.0` transition and later post-`1.0` SemVer behavior.

Upstream Kalshi changes do not automatically imply a crate major version bump.
The decision is based on whether downstream users of this crate need to change
their Rust code.

## Compatibility Tuple

Each release should record a compatibility block with:

- Docs snapshot date
- OpenAPI version
- AsyncAPI version
- Validated through changelog date

Example:

```text
Compatibility
- Docs snapshot: 2026-04-17
- OpenAPI: 3.13.0
- AsyncAPI: 2.0.0
- Validated through changelog: 2026-04-17
```

This compatibility tuple is the primary freshness signal for upstream Kalshi
alignment.

## Changelog Structure

`CHANGELOG.md` should remain readable as release history, not as a policy dump.
Prefer conventional sections:

- `Added`
- `Changed`
- `Deprecated`
- `Removed`
- `Fixed`

Add a `Compatibility` block for each release, and add a `Breaking` section when
breaking Rust API changes need to be called out explicitly.

Use short prefixes inside bullets when helpful:

- `[Rust API]`
- `[Upstream]`
- `[Docs]`
- `[CI]`
- `[Tests]`

## Version Bump Rules For Refreshes

When a refresh automation or contributor updates the repo against current
Kalshi docs:

- Bump `patch` if the work is limited to upstream additive alignment, docs/parity refreshes, internal fixes, tests, or CI, and the public Rust API remains source compatible.
- Bump `minor` if deprecated or removed upstream fields/endpoints force a breaking Rust API change, if public types or method names change, or if downstream consumers are likely to need code changes.
- Do not use the crate version as a proxy for the upstream Kalshi spec version.

## Automation Expectations

Repo refresh automations should:

1. Read `AGENTS.md`, this file, and the active `CHANGELOG.md` entry.
2. Check the Kalshi source-of-truth docs: `llms.txt`, changelog RSS, OpenAPI, and AsyncAPI.
3. Classify upstream changes as breaking or non-breaking.
4. Update `docs/spec-parity.md` when there is a spec-to-crate distinction
   worth documenting.
5. Remove or update stale deprecated endpoints, API shapes, fields, and tests.
6. Propose the version bump using the rules above.
7. Update `CHANGELOG.md`, README, and crate docs if the public contract or compatibility statement changed.

The step-by-step execution workflow for this repository lives in
`.codex/skills/kalshi-refresh/SKILL.md`.
