#[macro_use]
extern crate log;
extern crate env_logger;
extern crate git2;

mod consts;
mod git;
mod proc;
mod tui;

use git::update_repo;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    env_logger::init();

    let path = PathBuf::from(".");
    let mut branches = HashMap::new();
    branches.insert(String::from("master"), true);
    println!("Updating repo at {:#?}", path);
    update_repo(&path, "origin", &branches).unwrap();
}
