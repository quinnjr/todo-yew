// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

use serde::{ Deserialize, Serialize };

/// An entry in our todo list.
#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    /// Description of the thing to do.
    pub description: String,
    /// Whether the entry is completed.
    pub completed: bool,
    /// Whether the entry is being edited.
    pub editing: bool,
}
