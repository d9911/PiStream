#!/bin/sh
set -eu
project_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$project_dir"
cargo build --locked --bin pistream-cli
case "${CARGO_TARGET_DIR:-$project_dir/target}" in
    /*) target_dir="${CARGO_TARGET_DIR:-$project_dir/target}" ;;
    *) target_dir="$project_dir/$CARGO_TARGET_DIR" ;;
esac
scratch=$(mktemp -d "${TMPDIR:-/tmp}/pistream-smoke.XXXXXX")
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
cd "$scratch"
printf '6\n1000\ny\nn\n' | "$target_dir/debug/pistream-cli" > cli.log 2>&1
# Known decimal prefix; the full precision is covered independently by MPFR tests.
grep -q '^0:0:1000:14159265358979323846264338327950288419716939937510' pi_stream_1000_digits.txt
test -s pi_1000_digits_STREAMING.txt
awk -F: '!/^#/ && NF == 4 { count += length($4) } END { exit count != 1000 }' pi_stream_1000_digits.txt
printf 'OK: CLI streaming saved 1000 digits in an isolated temporary directory.\n'
