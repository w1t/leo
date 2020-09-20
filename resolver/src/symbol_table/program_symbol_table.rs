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

use crate::SymbolTable;
use leo_typed::Program;

/// Stores all function and circuit identifiers for a Leo program file
pub struct ProgramSymbolTable {
    program: Program,
    table: SymbolTable,
}

impl ProgramSymbolTable {
    /// Create a new program symbol table for a given Leo program file
    pub fn new(program: Program) -> Self {
        ProgramSymbolTable {
            program,
            table: SymbolTable::new(None),
        }
    }

    /// Store all program identifiers
    pub fn store_program(mut self) {
        self.store_circuits()
    }

    /// Store all circuit identifiers
    fn store_circuits(mut self) {
        self.program.circuits.into_iter().for_each(|(identifier, circuit)| {
            // Insert static circuit identifiers into table

            // self.table.insert()
        })
    }
}

pub struct hello();

impl hello {
    fn hello() -> Self {
        hello {}
    }
}
