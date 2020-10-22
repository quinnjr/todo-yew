// Copyright (C) Joseph R. Quinn
// SPDX-License-Identifier: MIT

use serde::{ Deserialize, Serialize };
use strum_macros::{ EnumIter, ToString };

use crate::entry::Entry;

#[derive(Debug, Clone, Copy, EnumIter, ToString, PartialEq, Deserialize, Serialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn fits(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into()
        }
    }
}