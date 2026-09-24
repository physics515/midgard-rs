# midgard-rs roadmap

A pure, phase-ordered task queue. Every work item is a `[ ]`/`[x]` checkbox. Tick `[x]` only when the
item genuinely shipped. Discovered work becomes a new `[ ]` in the right phase. No status tables, no
run log — git history, the PRs and the GitHub releases are the record.

Bootstrapped 2026-09-24 from a survey of the repo at `8ae8134`.

## Phase 0 — Make the crate buildable and releasable again (BLOCKER — nothing else can be verified first)

- [ ] **Restore standalone dependency versions in `Cargo.toml`.** The last commit (`8ae8134`, "merge
      into workspace", 2025-01-15) rewrote every dependency to `{ workspace = true }`, but this repo
      has no `[workspace]` table. `cargo metadata` fails with `error inheriting 'anyhow' from
      workspace root manifest's 'workspace.dependencies.anyhow' / failed to find a workspace root`,
      so master does not build, test, lint or publish at all. Take the concrete versions from
      `8a1c966:Cargo.toml` as the starting point and move each to the latest that builds green.
- [ ] Commit a refreshed `Cargo.lock`; `cargo build --locked` and `cargo test --locked` green.
- [ ] **Two dependencies are dead weight in the public manifest.** A green `cargo build --locked`
      reports `warning: unused dependency 'rand'` and `warning: unused dependency 'tokio-test'`
      (`cargo::unused_dependencies`). `tokio-test` is used only by doc-tests, so it belongs in
      `[dev-dependencies]`; `rand` looks genuinely unused — confirm and drop it. Both are currently
      hard runtime dependencies every consumer pays for. While there, check whether `tokio`'s `full`
      feature can drop to the features the crate actually uses.
- [ ] `cargo publish --locked --dry-run` green (nothing may be published until it is).
- [ ] `cargo +nightly fmt --check` is dirty on master today — pre-existing space-indented lines that
      predate this roadmap (`src/api/members/member_details.rs`, `src/api/savers/savers_details.rs`,
      `src/lib.rs`, and others) violate the repo's own `hard_tabs = true`. Land the reformat as its
      OWN commit, separate from any behavior change, so it doesn't bury a real diff — and do it
      before the CI item turns it into a permanent red check.
- [ ] Add CI under `.github/workflows/` — build + test + clippy + `cargo +nightly fmt --check` on
      push and PR. The repo has no CI today, so until this lands a PR has no checks to be green.

## Phase 1 — Correctness and honesty of the public surface

- [ ] **Docs contradict the code on the default base URL.** `README.md` and the `src/lib.rs` crate
      docs both say the default is `https://midgard.thorswap.net/v2/`; `Configuration::default()`
      (`src/midgard/config.rs`) uses `https://midgard.ninerealms.com/v2/`. Decide which is correct,
      fix the other, and say which way it went in the PR.
- [ ] **The crate has no offline test suite.** All 48 `#[test]`/`#[tokio::test]` functions and every
      doc-test call the live Midgard API, so a network outage looks identical to a broken crate. Add
      serde round-trip tests over captured JSON fixtures (checked into `tests/fixtures/`) so type
      changes can be verified without the network. Keep the live tests, but mark them so they can be
      selected separately.
- [ ] Fill in the `# Errors` doc sections — they are literally `todo` across the endpoint methods
      (e.g. `src/midgard/endpoints/health.rs`).
- [ ] Typo in `Cargo.toml` `keywords`: `migard` → `midgard`. Consumer-visible on crates.io.
- [ ] Public API returns `anyhow::Result` from every endpoint, which gives library consumers nothing
      to match on. Evaluate a concrete `thiserror` error enum (`thiserror` is already a dependency,
      and `src/types/errors.rs` exists) — this is a breaking change, so it takes a minor bump.

## Phase 2 — API coverage vs upstream Midgard v2

- [ ] Audit the implemented endpoints (`src/midgard/endpoints/`: actions, balance, borrowers, churn,
      global_stats, health, history, members, network, nodes, pools, savers, thorname) against the
      current Midgard v2 OpenAPI spec, and file each gap as its own `[ ]` item here with a link.
- [ ] Re-check the typed response structs against the live spec — the crate was last touched in
      January 2025 and THORChain has shipped since; a silently-changed field is a deserialization
      failure for consumers.

## Phase 3 — Ergonomics and modernization

- [ ] Every endpoint takes `&mut self` purely to update the rate-limit timestamp, so a `Midgard`
      cannot be shared across tasks without a lock. Evaluate interior mutability for the limiter.
- [ ] `edition = "2021"` → `2024`.
- [ ] Declare an MSRV (`rust-version`) and check it in CI.
- [ ] Add `documentation = "https://docs.rs/midgard-rs"` and a `homepage` to `Cargo.toml`; crates.io
      currently shows neither.
