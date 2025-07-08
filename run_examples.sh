#!/usr/bin/env bash

set -u
set -e
set -o pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

examples=('hyper_server_example'
	  'axum_server_example'
	  'actix_server_example'
	  'rocket_server_example'
	  'salvo_server_example'
          'poem_server_example')

for example in "${examples[@]}"; do
    echo "Running example for ${example}"
    pushd "$SCRIPT_DIR/$example" &> /dev/null || exit 1
    cargo build --release &> /dev/null
    cargo run --release &
    pid=$!
    trap 'kill -9 $pid &> /dev/null || true' EXIT
    sleep 1
    curl "http://localhost:3000/index.html" 2> /dev/null | grep "Hello from file"
    curl "http://localhost:3000/api" 2> /dev/null | grep "Hello from API"
    kill "$pid"
    echo "Binary size: $(du -sh "target/release/${example}")"
    popd &> /dev/null  || exit 2
    echo "======="
done
