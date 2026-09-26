//! Deterministic selection for the live endpoint tests.
//!
//! Several endpoint tests need *some* pool, member or borrower to call with,
//! and used to take one with `rand::thread_rng()`. That made every run a fresh
//! draw, which is how three real bugs stayed hidden for months: the negative
//! `units` in `PoolStatistics`, the missing transaction `height`, and the
//! negative `liquidityUnits` that made `get_member_details` fail roughly one
//! run in nine hundred. A test that fails only on an unlucky draw reads as
//! flaky, and flaky tests get re-run rather than investigated.
//!
//! The draw is therefore seeded and fixed by default: the same run picks the
//! same targets, so a failure reproduces and can be pinned as a fixture.
//! Setting `MIDGARD_TEST_SEED` to another value re-rolls the selection, which
//! is the deliberate way to go looking for payloads this crate cannot parse.

use rand::rngs::StdRng;
use rand::SeedableRng;

/// The default seed. Arbitrary, and only meaningful in that it does not change.
const DEFAULT_SEED: u64 = 0x6d69_6467_6172_64;

/// A generator seeded from `MIDGARD_TEST_SEED`, or [`DEFAULT_SEED`].
///
/// Hold the returned value for the whole of a test: a fresh generator restarts
/// the same sequence, so calling this twice hands back the same "random" index.
pub(crate) fn seeded_rng() -> StdRng {
	let seed = std::env::var("MIDGARD_TEST_SEED").ok().and_then(|raw| raw.parse::<u64>().ok()).unwrap_or(DEFAULT_SEED);
	StdRng::seed_from_u64(seed)
}
