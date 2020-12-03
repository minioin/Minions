#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

pub mod actions;
pub mod mcore;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod frontend;
