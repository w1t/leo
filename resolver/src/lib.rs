// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! A resolved syntax tree is represented as a `Program` without implicit type literals.
pub mod ast;
pub use self::ast::*;

pub mod errors;
pub use self::errors::*;

pub mod symbol_table;
pub use self::symbol_table::*;

use leo_typed::LeoTypedAst;

#[derive(Debug, Eq, PartialEq)]
pub struct LeoResolvedAst {
    // resolved_ast:
}

impl LeoResolvedAst {
    /// Creates a new resolved syntax tree from a given typed syntax tree
    pub fn new(program: LeoTypedAst) {}
}
