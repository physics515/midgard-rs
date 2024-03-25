# Midgard
Consumer information relating to swaps, pools, and volume. Midgard returns time-series information regarding the THORChain network, such as volume, pool information, users, liquidity providers and more. It also proxies to THORNode to reduce burden on the network. Runs on every node.

## midgard-rs
This crate aims to provide fully typed client for the THORChain Midgard API. 
* By default it references the `https://midgard.thorswap.net/v2/` base url but this can be changed by creating a new `Configuration` object and passing it to the `Midgard::with_config()` method.
* The client is rate limited to 1 request per second by default but this can be changed by creating a new `Configuration` object and passing it to the `Midgard::with_config()` method.

## Basic Usage

```rust
use midgard::Midgard;

#[tokio::main]
async fn main() {
    let midgard = Midgard::new();
    let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
    let balance = midgard.get_balance(address, None, None).await.unwrap();
    println!("coins: {:?}", balance.get_coins());
}
```

## Configuration

```rust
use midgard::Midgard;
use midgard::Configuration;

#[tokio::main]
async fn main() {
    let config = Configuration::new("https://midgard.ninerealms.com/v2/".to_string(), 1000); // base_url, rate_limit_ms
    let midgard = Midgard::with_config(config);
    let address = "thor102y0m3uptg0vvudeyh00r2fnz70wq7d8y7mu2g";
    let balance = midgard.get_balance(address, None, None).await.unwrap(); // address, timestamp, height
    println!("coins: {:?}", balance.get_coins());
}
```
