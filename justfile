help:
    just --list

binstall *args:
    @which cargo-binstall || cargo install cargo-binstall --locked
    cargo binstall -y "{{args}}"

[working-directory: "writings"]
readme:
    @which cargo-readme || just binstall cargo-readme
    cargo readme --output README.md

fix:
    cargo fix --allow-dirty --allow-staged

clean:
    cargo clean

check:
    cargo check --all-targets --all-features

test:
    cargo test --all-targets --all-features

release: fix check test
