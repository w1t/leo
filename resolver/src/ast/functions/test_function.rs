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

use crate::{Function, FunctionError, ResolvedNode, SymbolTable};
use leo_typed::{Identifier, TestFunction as UnresolvedTestFunction};

use serde::{Deserialize, Serialize};

/// A type checked test function
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestFunction {
    /// The test function definition
    pub function: Function,
    /// The custom test input file
    pub input_file: Option<Identifier>,
}

impl ResolvedNode for TestFunction {
    type Error = FunctionError;
    type UnresolvedNode = UnresolvedTestFunction;

    /// Return a type checked `TestFunction` given an unresolved test function
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        Ok(TestFunction {
            function: Function::resolve(table, unresolved.function).unwrap(),
            input_file: unresolved.input_file,
        })
    }
}
