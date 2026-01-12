#!/usr/bin/env bash

set -e

OUTFILE="scratch/testout.txt"

cargo run --release --quiet -- -t > "$OUTFILE"
printf "Data written to $OUTFILE.\n"
