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

> **Note on public instances.** `midgard.ninerealms.com`, which this crate used
> as its default through 0.0.5, stopped resolving entirely; `midgard.thorswap.net`
> answers non-browser clients with a Cloudflare challenge instead of JSON. If the
> gateway above stops working for you, check the THORChain dev docs linked above
> for the current list and pass it via `Configuration`.
