#!/usr/bin/env nu

def main [relative_dir: string, root: string] {
    $"relative_dir=($relative_dir)\n" | save -af $"($root)/target/debug/.set-filter.log"
    ($relative_dir | path basename) | save -f $"($root)/target/debug/.test_filter"
}
