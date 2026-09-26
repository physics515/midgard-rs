# Midgard
Consumer information relating to swaps, pools, and volume. Midgard returns time-series information regarding the THORChain network, such as volume, pool information, users, liquidity providers and more. It also proxies to THORNode to reduce burden on the network. Runs on every node.

## midgard-rs
A typed Rust client for the THORChain Midgard v2 API. Every response is
deserialized into a concrete struct rather than handed back as loose JSON, and
every failure is an [`APIError`](#errors) you can match on.

### Coverage

**25 of the 39 data endpoints**, checked against Midgard OpenAPI spec 2.35.0 on
2026-09-25. Implemented: actions, balance, borrower, borrowers, churns, health,
the depths / earnings / liquidity_changes / savers / swaps / tvl histories,
knownpools, member, members, network, nodes, pool, pool stats, pools, saver,
stats, and the three thorname lookups.

Not yet implemented — calling Midgard directly is the workaround until they land:
`/v2/bonds/{address}`, `/v2/holders`, `/v2/votes`,
`/v2/runepool/{address}`, `/v2/tcy/distribution/{address}`, the three
`/v2/history/affiliate*` endpoints, `/v2/history/reserve`, `/v2/history/rune`,
`/v2/history/runepool`, and the three `/v2/metrics/*` endpoints.

### Where your requests go

By default the client calls
`https://gateway.liquify.com/chain/thorchain_midgard/v2/` — a **third-party
public gateway**, the one [THORChain's developer documentation
recommends](https://dev.thorchain.org/concepts/connecting-to-thorchain.html),
rate limited to 50,000 requests per day per IP. Your queries reach Liquify's
infrastructure, not a node you control. That default is a convenience, not an
endorsement: point the client at your own Midgard instance, or any other public
one, with a `Configuration`. The default is exported as
`midgard_rs::DEFAULT_BASE_URL`.

The client is rate limited to 1 request per second by default, which
`Configuration` also controls.

## Basic Usage

```rust
use midgard_rs::Midgard;

#[tokio::main]
async fn main() {
    let mut midgard = Midgard::new();
    let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
    let balance = midgard.get_balance(address, None, None).await.unwrap();
    println!("coins: {:?}", balance.get_coins());
}
```

## Configuration

Point the client at any Midgard v2 instance — your own node, or another public
one — and set the rate limit to match what that instance allows.

```rust
use midgard_rs::Midgard;
use midgard_rs::Configuration;

#[tokio::main]
async fn main() {
    let config = Configuration::new("https://gateway.liquify.com/chain/thorchain_midgard/v2/".to_string(), 1000); // base_url, rate_limit_ms
    let mut midgard = Midgard::with_config(config);
    let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
    let balance = midgard.get_balance(address, None, None).await.unwrap(); // address, timestamp, height
    println!("coins: {:?}", balance.get_coins());
}
```

The default is exported as `midgard_rs::DEFAULT_BASE_URL` if you want to build a
`Configuration` around it.

## Errors

Every endpoint returns `Result<_, APIError>` — a concrete enum you can match
on, rather than an erased error:

| Variant | Means |
| --- | --- |
| `HttpStatus { status, url, body }` | The instance answered with a non-success status. The body is truncated to 512 characters. |
| `ReqwestError` | The request could not be made, or its body could not be read. |
| `SerdeError` | The response was not the JSON this crate expects. |
| `InvalidParameter` | The arguments were rejected before any request was made — `get_balance` with both a timestamp and a height, or a `history/*` call with `count` outside `1..=400`. |
| `QueryEncode` | A request's query string could not be encoded from the arguments given. |
| `ClientBuild` | The HTTP client could not be constructed. |

`APIError` is `#[non_exhaustive]`, so new variants are added without a breaking change.

## TLS

TLS is `rustls` with the bundled `webpki-roots` store; there is no OpenSSL in
the dependency graph. If you need certificates from your operating system's
trust store instead, depend on `reqwest` with `rustls-tls-native-roots` in your
own `Cargo.toml`.

## Testing this crate

Every request goes to a live Midgard instance, so most of the test suite needs
the network. The two halves are separated by module path: anything under
`midgard::endpoints::`, `midgard::tests::` or a `live_tests` module calls the
API, and everything else is pure (de)serialization over checked-in payloads.

```bash
cargo test --locked --lib -- --skip midgard::endpoints:: --skip midgard::tests:: --skip live_tests
```

CI runs that offline subset as a required check and the live suite as an
informational one, so a public instance being down cannot fail a pull request.

> **Note on public instances.** `midgard.ninerealms.com`, which this crate used
> as its default through 0.0.5, stopped resolving entirely; `midgard.thorswap.net`
> answers non-browser clients with a Cloudflare challenge instead of JSON. If the
> gateway above stops working for you, check the THORChain dev docs linked above
> for the current list and pass it via `Configuration`.
