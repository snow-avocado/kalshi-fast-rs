# Kalshi Refresh

Use this skill for recurring refreshes of `kalshi-fast-rs` against the current
Kalshi docs and changelog.

## Read First

Before making changes, read:

- `AGENTS.md`
- `VERSIONING.md`
- `CHANGELOG.md`
- `README.md`
- `src/lib.rs`

## Source Of Truth

Use these upstream documents as the source of truth:

- `https://docs.kalshi.com/llms.txt`
- `https://docs.kalshi.com/changelog/rss.xml`
- `https://docs.kalshi.com/openapi.yaml`
- `https://docs.kalshi.com/asyncapi.yaml`

Use this local note file when there is repo-specific context worth preserving:

- `docs/spec-parity.md`

## Workflow

1. Read the current changelog RSS and identify new or upcoming changes since the latest release or `Unreleased` entry.
2. Classify each relevant upstream change as breaking or non-breaking for:
   - the upstream Kalshi contract
   - the public Rust API of this crate
3. Check the live OpenAPI and AsyncAPI for schema drift relative to the current
   code, tests, and docs.
4. Update `docs/spec-parity.md` only for short human-facing notes about
   distinctions or behaviors that are not obvious from the YAML alone.
5. Search the codebase for deprecated endpoints, API shapes, fields, and examples that no longer match the current Kalshi docs.
6. Update code, tests, docs, and examples so the crate does not keep stale deprecated shapes around unnecessarily.
7. Prefer behavior and integration tests over generated parity artifacts when
   the YAML specs do not fully define runtime behavior.
8. Propose the crate version bump using `VERSIONING.md`:
   - patch for Rust API non-breaking refreshes
   - minor for Rust API breaking changes
9. Update `CHANGELOG.md`:
   - keep the active entry in normal changelog form
   - include a `Compatibility` block
   - use clear bullet prefixes like `[Rust API]`, `[Upstream]`, `[Docs]`, `[CI]`, or `[Tests]` where useful
10. Update `README.md` and `src/lib.rs` if the public compatibility statement or release/versioning references changed.
11. Run the relevant checks and summarize:
   - what changed upstream
   - whether the changes are breaking
   - what version bump is proposed
   - what code or docs were updated
   - any blockers or unresolved drift

## Expectations

- Do not leave obviously stale deprecated fields or endpoints in public structs, examples, or tests without documenting why.
- Prefer aligning the crate to the current Kalshi docs rather than preserving obsolete compatibility shims indefinitely.
- Keep the changelog useful for humans. Policy belongs in `VERSIONING.md`; release notes belong in `CHANGELOG.md`.

## Suggested Automation Prompt

Use this skill and keep the recurring prompt short. Example:

```md
Use [$kalshi-refresh](</Users/swe/repos/kalshi-rs/.codex/skills/kalshi-refresh/SKILL.md>).
Refresh the repo against the current Kalshi docs, classify new upstream changes
as breaking or non-breaking, update stale deprecated API shapes, propose the
appropriate version bump, and prepare the changelog/docs updates.
```
