// Opt in to unstable features expected for Rust 2018
//
#![feature(rust_2018_preview)]

// Opt in to warnings about new 2018 idioms
//
#![warn(rust_2018_idioms)]

#![feature(try_from)]

#![feature(nll)]

use serde_derive::{ Serialize, Deserialize };

mod gamepad;
mod config;
mod action;

pub use self::gamepad::*;
pub use self::config::*;
pub use self::action::*;
