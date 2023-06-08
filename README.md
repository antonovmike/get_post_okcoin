# stx_test

```bash
cargo run --bin check_n_transfer
```
```bash
cargo run --bin withdrawal_of_tokens
```

**Business requirements**

Standalone module that makes automated withdrawals of STX token on the balance from an account on okcoin exchange. 
Withdrawals are mode to a pre-saved STX address. 
There should be configurable options:
- if greater certain amount of STX on the balance then total available amount can be sent
- there will be 2 addresses to send to, module should use them in turn