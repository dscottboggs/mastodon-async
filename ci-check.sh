#!/bin/bash

set -euxo pipefail

cargo fix
cargo clippy --all-features -- -D warnings
cargo fmt
pushd entities
	cargo fix
	cargo clippy --all-features -- -D warnings
	cargo fmt
popd
cargo test --workspace --features=all
