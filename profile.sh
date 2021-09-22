#!/bin/sh

cargo build --release

perf record --call-graph=dwarf target/release/qrgen
perf script -F +pid > /tmp/test.perf

echo "View results on: https://profiler.firefox.com/"