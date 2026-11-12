#!/usr/bin/env bash
set -euo pipefail
project_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
cd "$project_root"
cargo test --locked --manifest-path fixtures/downstream-default/Cargo.toml
cargo check --locked --manifest-path fixtures/downstream-minimum/Cargo.toml
cargo test --locked --manifest-path fixtures/downstream-uom/Cargo.toml
cargo test --locked --manifest-path fixtures/feature-unification/Cargo.toml --workspace
