#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cargo install trunk' & \
 bash -c 'cd frontend; trunk build --address 0.0.0.0' & \
 bash -c 'cargo run --release')