# Week-01/Day-01 solution

Convert years to days.

### Develop

```
nix develop
```

#### Run Tests

```
cargo test
```

#### Run command-line app

```
cargo run
```

#### Build Documentation

```
cargo doc
```

#### Develop Web

```
nix shell nixpkgs#rustup
rustup default
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
~/.cargo/bin/trunk serve
```
