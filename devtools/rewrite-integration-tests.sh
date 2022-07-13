#!/bin/bash

set -euo pipefail

SCRIPT="$(readlink -f "$0")"
PROJDIR="$(dirname "$(dirname "$SCRIPT")")"

cd $PROJDIR
cargo build

cd ./integration-tests/src

for casedir in test-cases/*
do
  echo "Updating $casedir..."
  input="$(ls "$casedir"/input* | head -1)"
  if sappho eval "$input" > "$casedir/expected" 2>&1
  then
    # It successfully parsed and evaluated, so do source code rewrites:
    echo "Updating $casedir unparse expectations..."
    cp "$input" "$input.tmp"
    for style in canonical reduced
    do
      sappho parse -f "$style" "$input.tmp" > "$casedir/input-$style" || true
    done
    rm "$input.tmp"
  fi
done
