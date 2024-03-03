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
cargo run --example cli
```

#### Build Documentation

```
cargo doc
```

Open `target/doc/convert_ages_to_days_brainrake/index.html`.

#### Develop Web

```
nix develop
nix shell nixpkgs#rustup
rustup default
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
~/.cargo/bin/trunk serve examples/web/index.html --open
```
