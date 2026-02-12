#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Setup test fixtures
"$SCRIPT_DIR/setup_ef_tests.sh"

# Run EF tests
cargo nextest run --no-fail-fast -p ef-tests --release --features "asm-keccak ef-tests"
