#!/bin/sh
set -e
echo "Passed in dir: "
echo "$1"
cd "$1"
# /var/home/ajsnow/programs/github/PlaygroundsParent/rust_playground/target/debug/deps/rust_playground-<blah>
BIN=$(cargo test --no-run --message-format=json 2>/dev/null \
  | grep '"executable"' \
  | grep -oP '(?<="executable":")[^"]+' \
  | head -1)
ln -sf "$BIN" target/debug/deps/rust_playground_test
