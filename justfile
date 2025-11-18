native-crate := "kanash"
wasm-crate := "kanash-ratzilla"

[private]
default:
    @just --list --justfile {{justfile()}}

# target only native package
[group("build")]
build-native:
    cargo build -p {{native-crate}}

# target only wasm package
[group("build")]
build-wasm:
    cargo build --target wasm32-unknown-unknown -p {{wasm-crate}}

# build all
[group("build")]
build:
    @just build-native
    @just build-wasm

# Find TODOs and comments silencing lints
[group('tooling')]
todo:
    grep --recursive --extended-regexp --ignore-case --line-number --color=always 'noqa|todo' --exclude-dir target
