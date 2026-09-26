# midgard-rs roadmap

A pure, phase-ordered task queue. Every work item is a `[ ]`/`[x]` checkbox. Tick `[x]` only when the
item genuinely shipped. Discovered work becomes a new `[ ]` in the right phase. No status tables, no
run log — git history, the PRs and the GitHub releases are the record.

Bootstrapped 2026-09-24 from a survey of the repo at `8ae8134`.

## Phase 0 — Make the crate buildable and releasable again (BLOCKER — nothing else can be verified first)

- [x] **Restore standalone dependency versions in `Cargo.toml`.** The last commit (`8ae8134`, "merge
      into workspace", 2025-01-15) rewrote every dependency to `{ workspace = true }`, but this repo
      has no `[workspace]` table, so `cargo metadata` failed outright and master did not build, test,
      lint or publish at all.
- [x] Commit a refreshed `Cargo.lock`; `cargo build --locked` and `cargo test --locked` green.
- [x] Move `tokio-test` into `[dev-dependencies]`, along with `rand`, and drop `tokio`'s `full`
      feature down to the `time` the library actually uses.
- [x] `cargo publish --locked --dry-run` green.
- [x] Add CI under `.github/workflows/` — build (stable + nightly) + clippy + `cargo +nightly fmt
      --check` + an offline test job, with the live-network tests split into a separate
      `continue-on-error` job so a public instance being down cannot fail a PR.

## Phase 1 — Correctness and honesty of the public surface

- [x] **The default base URL pointed at a host that no longer exists.** `midgard.ninerealms.com` has
      no A, AAAA or CNAME record as of 2026-09-24, and the `midgard.thorswap.net` the docs advertised
      answers non-browser clients with a Cloudflare challenge. Now
      `https://gateway.liquify.com/chain/thorchain_midgard/v2/`, exported as `DEFAULT_BASE_URL`.
      Source: <https://dev.thorchain.org/concepts/connecting-to-thorchain.html>
- [x] **The API layer never checked the HTTP status.** A 404 or an error page went straight to
      `serde_json` and surfaced as a parse error. All 25 call sites now share
      `src/api/http.rs::get_body`, which checks the status and raises `APIError::HttpStatus`.
- [x] Fill in the `# Errors` doc sections on all 25 endpoint methods.
- [x] Typo in `Cargo.toml` `keywords`: `migard` → `midgard`.
- [x] **`reqwest`'s default features linked OpenSSL.** Switched to `rustls-tls` with
      `default-features = false`; no `openssl*` or `native-tls` remains in the graph.

- [ ] **Hold one `reqwest::Client` on `Midgard`.** Each request still builds its own client,
      connection pool and TLS configuration. A process-wide `static` client was tried on
      2026-09-24 and reverted: a `Client`'s pooled connections are driven by background tasks
      owned by whichever tokio runtime first used them, so after that runtime is dropped every
      later request fails with "runtime dropped the dispatch task". The client must therefore be
      owned by `Midgard`, whose lifetime a consumer controls. Note `Midgard` derives
      `Serialize`/`Deserialize`, so the field needs `#[serde(skip)]`, and all 25 `api_*` signatures
      grow a `&Client` parameter. See the write-up in `src/api/http.rs`.
- [ ] **`ring` is not pure Rust.** Moving to `rustls-tls` removed the dependency on a *system* C
      library, but `ring` still vendors C and generated assembly. A fully-Rust crypto backend
      (`rustls` with a RustCrypto provider) is the only way to satisfy the no-FFI principle
      literally; evaluate whether the maturity is there yet.
- [ ] **The crate has no fixture-based test suite.** 44 of the 79 lib tests still call the live
      Midgard API, so a network outage looks like a broken crate. The offline share has gone from
      0 to 35 by pinning the real payloads that were failing, but the endpoint tests themselves
      should run against captured JSON in `tests/fixtures/`. The network/offline split is already
      mechanised — network tests live under `midgard::endpoints::`, `midgard::tests::` or a
      `live_tests` module, and CI keys off exactly that.
- [ ] **Re-check the default base URL on every run.** It is a single point of failure outside this
      project's control and has already broken once. Liquify rate-limits to 50,000 requests/day/IP.
- [x] Public API returns `Result<_, APIError>` instead of `anyhow::Result`, so consumers can match
      on the failure. `APIError::QueryEncode` was added for the query-string errors `anyhow` used to
      absorb, and `anyhow` is dropped from the dependency set. Shipped in 0.2.0.
- [x] `Pool`'s silent NaN-to-zero coercion is gone — and it was worse than `lpLuvi`: the same
      helper covered 23 fields and swallowed *every* parse error into `0`, not just `"NaN"`. A live
      scan of all 43 pools showed only `lpLuvi` and `saversYieldShare` ever carry `"NaN"`, so the
      seven ratio-shaped fields are now `Option<Decimal>` and the other sixteen parse strictly.
      Shipped in 0.2.0.

## Phase 2 — API coverage vs upstream Midgard v2

Audited 2026-09-24 against the upstream OpenAPI spec (`openapi/openapi.yaml` on the `develop` branch
of <https://gitlab.com/thorchain/midgard>, spec version **2.35.0**) and against live responses. The
crate implements 24 of the 38 documented `/v2/*` paths.

- [x] `Pool` was missing `depthMinus2Percent`, `depthPlus2Percent` (Midgard 2.34.0),
      `liquidityInUSD` (2.34.1) and `saversYieldShare`.
- [x] `DepthHistoryInterval` was missing the OHLC set added in Midgard 2.33.0.
- [x] `ActionType` was missing `thorname`, `send`, `runePoolDeposit`, `runePoolWithdraw` (all in the
      spec) and `trade`, `contract` (emitted live, absent from the spec). An unknown value used to
      fail the whole `/v2/actions` response and now lands in `ActionType::Other`.
- [ ] Add `/v2/history/affiliate`, `/v2/history/affiliate/stats` and `/v2/history/affiliate/earnings`
      — affiliate volume history landed in Midgard 2.33.0 (2025-11-27), affiliate earnings in 2.34.0
      (2025-12-15). Source: <https://gitlab.com/thorchain/midgard/-/releases>
- [ ] Add `/v2/holders` — the Holders API landed in Midgard 2.32.8 (2025-08-28).
- [ ] Add `/v2/history/runepool` and `/v2/runepool/{address}`.
- [ ] Add `/v2/history/reserve` and `/v2/history/rune`.
- [ ] Add `/v2/bonds/{address}` and `/v2/votes`.
- [ ] Add `/v2/metrics/scores`, `/v2/metrics/data` and `/v2/metrics/anomalies`.
- [ ] Add `/v2/tcy/distribution/{address}`.
- [ ] `ActionMetadata` has fields only for `swap`, `addLiquidity`, `withdraw` and `refund`. Live
      `/v2/actions` also returns a `contract` metadata block, and the new action types will bring
      their own. Nothing fails — the struct does not use `deny_unknown_fields` — but the data is
      silently dropped.
- [ ] `genesisInfo` is no longer returned by `/v2/health`. `HealthInfo::genesis_info` is `Option` so
      it still parses, but `get_health_info`'s doc comment documents it as a live field.
- [x] `ActionOut::height` was a required `u64`, but `height` is optional on the spec's `Transaction`
      schema and genuinely absent for any outbound that has not landed in a block — 31 of the 40
      outbounds in a plain `/v2/actions?limit=50`. It failed the whole actions response. Now
      `Option<u64>`, on `ActionIn` too, along with the spec's optional `affiliate` flag.
- [x] **`get_member_details`'s intermittent `invalid digit found in string` is fixed.** Sampling
      randomly across all 15,115 members and checking every `u64`-typed field programmatically found
      it on the first pass: `"liquidityUnits": "-200450712"` on `LTC.LTC`, a member who has withdrawn
      more than they added. One payload in ~900, which is why the randomised test caught it only
      sometimes. `MemberPool::liquidity_units` and the identical latent case in
      `PoolStatistics::liquidity_units` are now `i64`, with the payload pinned as a regression test.
      Shipped in 0.2.0.
- [x] **The randomised endpoint tests are deterministic.** The draw is seeded from a constant via
      `test_support::seeded_rng`, so a run picks the same targets and a failure reproduces;
      `MIDGARD_TEST_SEED` re-rolls it to go hunting deliberately. The five doc examples that picked
      at random now use a named pool or the first list entry. Shipped in 0.2.0.
- [ ] Sweep the remaining response types for `u64` fields that upstream can report as negative, the
      way `units`/`synthUnits`/`blockRewards` were. `savers`, `swaps`, `tvl`, `members`,
      `borrowers`, `nodes`, `network` and `stats` were checked against live data on 2026-09-24 and
      are clean, but the check should be a fixture test, not a one-off.

## Phase 3 — Ergonomics and modernization

- [ ] Every endpoint takes `&mut self` purely to update the rate-limit timestamp, so a `Midgard`
      cannot be shared across tasks without a lock. Evaluate interior mutability for the limiter.
      Pairs naturally with holding a `Client` on `Midgard`.
- [ ] `edition = "2021"` → `2024`.
- [ ] Declare an MSRV (`rust-version`) and check it in CI. The crate builds on stable 1.98.0 today.
- [x] Add `documentation = "https://docs.rs/midgard-rs"` to `Cargo.toml`. (A `homepage` was tried
      and dropped — cargo warns that it is redundant with `repository` when they are the same URL.)
- [ ] Dependencies a major version or more behind, checked against crates.io on 2026-09-24:
      `thiserror` 1 → **2.0.21**, `reqwest` 0.12 → **0.13.5**, `rand` 0.8 → **0.10.3** (`thread_rng`
      and `gen_range` were renamed in 0.9, which touches the test modules). `serde_with` (3.23.0),
      `rust_decimal` (1.43.0), `serde-aux` and `chrono` are all current within their major.

## Routine hygiene

- [ ] **The single-instance PID lock does not hold in this sandbox.** The routine writes the PID of
      a detached `sleep` to `scratch/midgard-routine/routine.lock`, but detached background
      processes are reaped almost immediately here, so the lock reads as stale within seconds. On
      2026-09-24 a second session took it over and ran `cargo clippy` in this same working tree
      mid-run. Anchor the lock to the owning `claude` session PID, or to a heartbeat timestamp.
- [ ] **Builds on this host are I/O-bound, not CPU-bound.** `~/.cargo/config.toml` (created
      2026-09-24 by another routine) points every repo under `/mnt/deepmem/Development` at one
      shared `Cargo Target` directory, so six concurrent nightly builds serialise on a single
      `.cargo-build-lock`; on top of that, rustc stalls in `balance_dirty_pages` waiting on btrfs
      writeback. A cold build took 23m30s that way and under 15s with `CARGO_TARGET_DIR` on the
      `/tmp` tmpfs. Decide deliberately whether the shared target dir is worth it.
- [ ] That same `~/.cargo/config.toml` sets `rustflags = ["-Z", "threads=8"]` globally, which is
      nightly-only: any `cargo +stable` invocation needs `RUSTFLAGS=` to override it, or it fails.
      Since this crate must build on stable, that is a trap worth removing or scoping.
