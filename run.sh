#!/bin/sh

set -x

function run_local(){
  pasuspender /usr/bin/jackd -- -dalsa -dhw:0 -r48000 -p128 -n2 -Xseq &
  sleep 1
  cargo run
  kill %1
}

function run_local_release(){
  pasuspender /usr/bin/jackd -- -dalsa -dhw:0 -r48000 -p128 -n2 -Xseq &
  cargo build --release
  sleep 1
  target/release/nui-1
  kill %1
}

case "$1" in
  ""|"help")
    cat <<EOF
$0 [local|local-release|pi]

local         -- run local build and jack connections
local-release -- same in release mode
pi            -- run compiled version in a raspberry pi
EOF
    exit 1
    ;;
  "local")
    run_local
    ;;
  "local-release")
    run_local_release
    ;;
  "pi")
    run_pi
    ;;
esac
