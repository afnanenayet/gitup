#![recursion_limit = "1024"]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate git2;
#[macro_use]
extern crate error_chain;

mod consts;
mod git;
mod proc;
mod tui;

use git::update_repo;
use std::collections::HashMap;
use std::path::PathBuf;

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

pub use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    env_logger::init();

    let path = PathBuf::from(".");
    let mut branches = HashMap::new();
    branches.insert(String::from("master"), true);
    println!("Updating repo at {:#?}", path);
    update_repo(&path, "origin", &branches).unwrap();
    Ok(())
}
