#!/usr/bin/env bash
# S4: 100% line and function coverage from real execution tracing (main.rs and controller.rs are I/O wiring, excluded; every decision lives in pure functions).
set -euo pipefail
cargo +1.98.0 llvm-cov --ignore-filename-regex 'main\.rs|controller\.rs' --fail-under-lines 100 --fail-under-functions 100
