#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -ex

exec watchexec \
  --shell=none \
  --project-origin . -w src -w Cargo.toml \
  --exts rs,toml \
  -r \
  -- cargo build
