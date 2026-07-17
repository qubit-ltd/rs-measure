#!/bin/bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
FMT_TOOLCHAIN="${RS_CI_FMT_TOOLCHAIN:-nightly-2026-06-05}"

env RS_CI_PROJECT_ROOT="$PROJECT_ROOT" "$PROJECT_ROOT/.rs-ci/align-ci.sh" "$@"

cargo +"$FMT_TOOLCHAIN" fmt \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-default/Cargo.toml" \
    -- --config-path "$PROJECT_ROOT/.rs-ci/rustfmt.toml"
cargo +"$FMT_TOOLCHAIN" fmt \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-uom/Cargo.toml" \
    -- --config-path "$PROJECT_ROOT/.rs-ci/rustfmt.toml"
cargo +"$FMT_TOOLCHAIN" fmt --all \
    --manifest-path "$PROJECT_ROOT/fixtures/feature-unification/Cargo.toml" \
    -- --config-path "$PROJECT_ROOT/.rs-ci/rustfmt.toml"
cargo +"$FMT_TOOLCHAIN" fmt \
    --manifest-path "$PROJECT_ROOT/fixtures/downstream-minimum/Cargo.toml" \
    -- --config-path "$PROJECT_ROOT/.rs-ci/rustfmt.toml"
