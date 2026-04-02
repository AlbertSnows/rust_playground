#!/usr/bin/env nu

  let root = $env.ZED_WORKTREE_ROOT  # or take as arg
  let bin = (
      cargo test --no-run --message-format=json 2>/dev/null
      | lines
      | each { from json }
      | where executable? != null
      | first
      | get executable
  )

  ln -sf $bin $"($root)/target/debug/deps/rust_playground_test"

  if ($env | get -o ZED_RELATIVE_DIR) != null {
      let module = ($env.ZED_RELATIVE_DIR | path basename)
      $module | save -f $"($root)/target/debug/.test_filter"
  }
