# get_post_okcoin

**Business requirements**

Standalone module that makes automated withdrawals of STX token on the balance from an account on okcoin exchange. 
Withdrawals are mode to a pre-saved STX address. 
There should be configurable options:
- if greater certain amount of STX on the balance then total available amount can be sent
- there will be 2 addresses to send to, module should use them in turn

[okcoin API](https://www.okcoin.com/docs-v5/)

**Your account details**

Personal data (key, secret key and password) are stored in the ".env" file. For example:

```bash
OKCOIN_API_KEY=u1uuuu1u-1111-1u11-11u1-111uu1111111
OKCOIN_API_SECRET=V48T709QV84YTS9YSE9
OKCOIN_PASS_PHRASE=your_password
```

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
$ git clone https://github.com/antonovmike/get_post_okcoin.git
$ cd stacks-blockchain
```

### 3. Run the project
```bash
cargo run -- config.toml
```

### 3. Run the binary
```bash
cargo run --bin get_post_okcoin
```

## Testing

**Run the tests:**

```bash
$ cargo test
```

Debug:
```bash
RUST_LOG=debug cargo run -- config.toml
```