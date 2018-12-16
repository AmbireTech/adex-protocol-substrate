# adex-protocol-substrate

The [AdEx Protocol](https://github.com/AdExNetwork/adex-protocol) implemented on top of Substrate. Bootstrapped from [substrate-node-template](https://github.com/paritytech/substrate-node-template).


## What is it?

### OUTPACE

The [`AdExOUTPACE`](https://github.com/AdExNetwork/adex-protocol-substrate/tree/rebase/runtime/src/adex_outpace) module implements **O**ffchain **U**nidirectional **T**rustless **Pa**yment **C**hann**e**ls described here: https://github.com/AdExNetwork/adex-protocol/blob/master/OUTPACE.md

The OUTPACE module consists of:

* `channel_open`: opens a channel, therefore locking up a deposit
* `channel_withdraw_expired`: after the channel is expired, the creator may invoke this to withdraw the remainder of their deposit
* `channel_withdraw`: at any time before expiry, anyone who earned from this channel may withdraw their earnings

### Registry

The upcoming AdExRegistry module implements the AdEx registry.

It is a component where AdEx validators can stake tokens to get exposure. Furhermore, users may launch challenges against validators to prove their misbehavior. Most of the challenges involve replicating the off-chain behavior of the [validator stack](https://github.com/adexnetwork/adex-validator-stack-js), employing a pattern referred to as counterfactuality.

For more details, read https://github.com/AdExNetwork/adex-protocol/issues/7

## Build and run

```
cargo run -- --dev
```

With some old Rust crates, you might need to do `export PKG_CONFIG_PATH=/usr/lib/openssl-1.0/pkgconfig` if you're running OpenSSL 1.1


## Bootstrap an UI

First, complete the "Prerequisites" step from https://substrate.readme.io/docs/creating-a-custom-substrate-chain

Then, execute:

```
substrate-ui-new adex-protocol-substrate
```
