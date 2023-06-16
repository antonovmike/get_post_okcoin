# get_post_okcoin

**Business requirements**

Standalone module that makes automated withdrawals of STX token on the balance from an account on okcoin exchange. 
Withdrawals are mode to a pre-saved STX address. 
There should be configurable options:
- if greater certain amount of STX on the balance then total available amount can be sent
- there will be 2 addresses to send to, module should use them in turn

[okcoin API](https://www.okcoin.com/docs-v5/)

## Building

### 1. Download and install Rust

_For building on Windows, follow the rustup installer instructions at https://rustup.rs/._

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ rustup component add rustfmt
```

- When building the [`main`](https://github.com/antonovmike/get_post_okcoin/tree/main) branch, ensure you are using the latest stable release:

```bash
$ rustup update
```

### 2. Clone the source repository:

```bash
$ git clone --depth=1 https://github.com/antonovmike/get_post_okcoin.git
$ cd stacks-blockchain
```

### 3. Build the project

```bash
$ cargo build
```