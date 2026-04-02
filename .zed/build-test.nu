#!/usr/bin/env nu

def main [root: string, relative_dir?: string] {
    cd $root
    let bin = (
        cargo test --no-run --message-format=json 2>/dev/null
        | lines
        | each { from json }
        | where executable? != null
        | first
        | get executable
    )

    ln -sf $bin $"($root)/target/debug/deps/rust_playground_test"

    if $relative_dir != null {
        let module = ($relative_dir | path basename)
        $module | save -f $"($root)/target/debug/.test_filter"
    }
}
