#!/bin/bash
set -efuo pipefail

function run {
  echo
  echo '```'
  echo "\$ $*"
  eval "$@"
  echo '```'
}

run cd usercode
run $'RUSTFLAGS=\'--cfg procmacro2_semver_exempt\'' cargo expand --tests --lib
run $'RUSTFLAGS=\'--cfg procmacro2_semver_exempt\'' cargo build --tests --lib
run cargo version --verbose
