#!/bin/bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
BUILD_TOOLCHAIN="${RS_CI_BUILD_TOOLCHAIN:-1.94.0}"
CARGO_TARGET_DIR="$PROJECT_ROOT/target/downstream-fixtures"

CARGO_TARGET_DIR="$CARGO_TARGET_DIR" cargo +"$BUILD_TOOLCHAIN" check --locked \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-default/Cargo.toml"
CARGO_TARGET_DIR="$CARGO_TARGET_DIR" cargo +"$BUILD_TOOLCHAIN" test --locked \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-default/Cargo.toml"
CARGO_TARGET_DIR="$CARGO_TARGET_DIR" cargo +"$BUILD_TOOLCHAIN" test --locked \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-uom/Cargo.toml"

exec env RS_CI_PROJECT_ROOT="$PROJECT_ROOT" "$PROJECT_ROOT/.rs-ci/ci-check.sh" "$@"
