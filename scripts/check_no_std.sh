#!/usr/bin/env bash
set -eo pipefail

target=riscv32imac-unknown-none-elf

cmd=(cargo +stable build --no-default-features --target "$target" -p stateless)

echo "Running: ${cmd[*]}"
"${cmd[@]}"
