# to test: 
cargo run <module>

cfg = configuration

#[cfg(test)] = only compile this module when running tests, not in release

assert!(x)               // x must be true
assert!(!x)              // x must be false
assert_eq!(a, b)         // a must equal b — shows both values on failure
assert_ne!(a, b)         // a must not equal b

# how does the debugger work?

cargo compiles test binaries to a path with a has in the name. e.g.
  target/debug/deps/rust_playground-a3f9c2b1e4d7

because of the hash, we can't hardcode the path to the binaries. We need as tbale way to point to the latest binary.

# steps
- trigger the debug tests in zed
- - zed reads debug.json, sees "build" step before launching. runs that first. 

# what does build-test.sh do?

main line is BIN=

cargo test --no-run -> compiles the test binary, but doesn't run it.
2>/dev/null -> discards stderr (ensures only json is outputted)
generates something like: 
{"reason":"compiler-artifact","package_id":"registry+https://github.com/rust-lang/crates.io-index#memchr@2.8.0","manifest_path":"/var/home/ajsnow/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.0/Cargo.toml","target":{"kind":["lib"],"crate_types":["lib"],"name":"memchr","src_path":"/var/home/ajsnow/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.0/src/lib.rs","edition":"2021","doc":true,"doctest":true,"test":true},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":["alloc","std"],"filenames":["/var/home/ajsnow/programs/github/PlaygroundsParent/rust_playground/target/debug/deps/libmemchr-ec5c298b979d44cc.rlib","/var/home/ajsnow/programs/github/PlaygroundsParent/rust_playground/target/debug/deps/libmemchr-ec5c298b979d44cc.rmeta"],"executable":null,"fresh":true}

grep executable keeps lines with an executable. e.g.
{"reason":"compiler-artifact","package_id":"registry+https://github.com/rust-lang/crates.io-index#memchr@2.8.0","manifest_path":"/var/home/ajsnow/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.0/Cargo.toml","target":{"kind":["lib"],"crate_types":["lib"],"name":"memchr","src_path":"/var/home/ajsnow/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.0/src/lib.rs","edition":"2021","doc":true,"doctest":true,"test":true},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":["alloc","std"],"filenames":["/var/home/ajsnow/programs/github/PlaygroundsParent/rust_playground/target/debug/deps/libmemchr-ec5c298b979d44cc.rlib","/var/home/ajsnow/programs/github/PlaygroundsParent/rust_playground/target/debug/deps/libmemchr-ec5c298b979d44cc.rmeta"],"executable":null,"fresh":true}

grep -oP '(?<="executable":")[^"]+'

extracts the binary path. uses regex. -o is only the match. -P is perl regex. reads as, "after 'executable':', capture everything that isn't a '
e.g.
/var/home/ajsnow/programs/github/PlaygroundsParent/rust_playground/target/debug/deps/rust_playground-b0687c4be048a400

head -1 just takes the first result



ln -sf creates a system link named rust_playground_test pointing to the hashed binary. 
-s = symbolic link
-f = force overwrite

# zed lanches the debugger

zed worktree root is the project root. it launches the binary directly for codelldb. 
test-threads 1 forces sequential test execution. otherwise there would be threads in parallel to track. 

  CodeLLDB is an LLDB-based adapter. LLDB is a native debugger (like GDB) that understands Rust's types and can step through code, inspect variables, set breakpoints, etc.


# questions to follow up on:
what are binaries?
binaries are compiled machine code. runnable by the os. 

what is an executable in the context of cargo test?
cargo tests complies code under #[test] into a single binary.

the executable in the json output is compiled binary. cargo puts intermediate build artifacts in there. we skip running the executable so zed can run it with codelldb

more reading:
- cargo test behavior: cargo test --help or the https://doc.rust-lang.org/cargo/commands/cargo-test.html
- https://doc.rust-lang.org/book/ch11-02-running-tests.html
- CodeLLDB: its https://github.com/vadimcn/codelldb has accurate docs
- Zed debug config: https://zed.dev/docs/debugger
