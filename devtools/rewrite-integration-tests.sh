#!/bin/bash

set -euo pipefail

SCRIPT="$(readlink -f "$0")"
PROJDIR="$(dirname "$(dirname "$SCRIPT")")"

cd $PROJDIR
cargo build

cd ./integration-tests/src

set -x
for casedir in test-cases/*
do
  input="$(ls "$casedir"/input* | head -1)"
  cp "$input" "$input.tmp"
  sappho eval "$input.tmp" > "$casedir/expected" 2>&1 || true
  for style in canonical reduced
  do
    sappho parse -f "$style" "$input.tmp" > "$casedir/input-$style" || true
  done
  rm "$input.tmp"
done
set +x
