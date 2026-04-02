#!/usr/bin/env nu

def main [root: string, relative_dir?: string] {
    cd $root

    $"root=($root) relative_dir=($relative_dir)\n" | save -af $"($root)/target/debug/.build-test.log"

    if $relative_dir != null {
        let module = ($relative_dir | path basename)
        $module | save -f $"($root)/target/debug/.test_filter"
    }

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
