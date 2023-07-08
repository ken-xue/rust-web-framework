# Code 
> Auto generated code

# Build

```bash
cargo build --release
```

clean

```bash
cargo clean
```

add to path

```bash
CODE_PATH=$(pwd)
echo $CODE_PATH
export PATH="$PATH:$CODE_PATH/target/release"
```

# Use cases

1. generate model code

```bash
code -u mysql://root:123456@localhost/rwf -m system -t sys_user -p ./target/ --prefix sys_
```

```bash
code -u mysql://root:123456@localhost/rwf -m system -t sys_role -p /Users/biaoyang/CLionProjects/rust-web-framework/server/src --prefix sys_ 
```

```bash
code -u mysql://root:123456@localhost/rwf -m system -t sys_menu -p /Users/biaoyang/CLionProjects/rust-web-framework/server/src --prefix sys_ 
```