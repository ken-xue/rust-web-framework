# Code 
> Auto generated code

# Build

```bash
cargo build --release --bin rwf --out-dir target/release
```

# Use cases

1. generate model code

```bash
./target/debug/code -u mysql://root:123456@localhost/rwf -m system -t sys_user -p ./target/ --prefix sys_
```