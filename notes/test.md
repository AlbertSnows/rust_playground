to test: 
cargo run <module>

cfg = configuration

#[cfg(test)] = only compile this module when running tests, not in release

assert!(x)               // x must be true
assert!(!x)              // x must be false
assert_eq!(a, b)         // a must equal b — shows both values on failure
assert_ne!(a, b)         // a must not equal b
