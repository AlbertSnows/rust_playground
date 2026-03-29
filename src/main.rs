#![allow(dead_code)]
#![allow(clippy::let_and_return)]
#![allow(unused_imports)]
#![warn(clippy::pedantic)]
mod common;
mod neetcode;
use log::info;

#[must_use]
pub fn get_foo() -> String {
    // can use lldb commands
    info!("It works!");
    let x = 1;
    info!("x: {x}");
    "foo".to_string()
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let _ = env_logger::try_init();
        assert_eq!(get_foo(), "foo".to_string());
    }
}
