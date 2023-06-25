# Code 
> Auto generated code

# Build

```bash
cargo build --release --bin rwf --out-dir target/release
```

# Use cases

1. generate model code

```bash
rwf -u mysql://root:123456@localhost/rwf -t sys_user -m user -p \
/Users/CLionProjects/rust-web-framework/server/src/system
```