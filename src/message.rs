// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

use crate::filter::Filter;

pub enum Message {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
}