#!/usr/bin/env nu

def main [root: string, relative_dir?: string] {
    cd $root

    cargo clean -p rust_playground
    let bin = (
        cargo test --no-run --message-format=json 2>/dev/null
        | lines
        | each { from json }
        | where executable? != null
        | first
        | get executable
    )

    ln -sf $bin $"($root)/target/debug/deps/rust_playground_test"
}
