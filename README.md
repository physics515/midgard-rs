# Midgard
Consumer information relating to swaps, pools, and volume. Midgard returns time-series information regarding the THORChain network, such as volume, pool information, users, liquidity providers and more. It also proxies to THORNode to reduce burden on the network. Runs on every node.

## midgard-rs
This crate aims to provide fully typed client for the THORChain Midgard API.
* By default it references the `https://gateway.liquify.com/chain/thorchain_midgard/v2/` base url — the public gateway [recommended by the THORChain developer documentation](https://dev.thorchain.org/concepts/connecting-to-thorchain.html) — but this can be changed by creating a new `Configuration` object and passing it to the `Midgard::with_config()` method.
* The client is rate limited to 1 request per second by default but this can be changed by creating a new `Configuration` object and passing it to the `Midgard::with_config()` method.

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

Every endpoint returns `anyhow::Result`, and the underlying error is an
`APIError`:

| Variant | Means |
| --- | --- |
| `HttpStatus { status, url, body }` | The instance answered with a non-success status. The body is truncated to 512 characters. |
| `ReqwestError` | The request could not be made, or its body could not be read. |
| `SerdeError` | The response was not the JSON this crate expects. |
| `InvalidParameter` | The arguments were rejected before any request was made — `get_balance` with both a timestamp and a height, or a `history/*` call with `count` outside `1..=400`. |
| `ClientBuild` | The HTTP client could not be constructed. |

`APIError` is `#[non_exhaustive]`.

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
