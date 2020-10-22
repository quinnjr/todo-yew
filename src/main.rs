// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

#![recursion_limit="512"]

pub mod entry;
pub mod filter;
pub mod message;
pub mod model;
pub mod state;

pub const KEY: &str = "quinnjr.todomvc.self";

pub fn main() {
    yew::start_app::<model::Model>();
}
