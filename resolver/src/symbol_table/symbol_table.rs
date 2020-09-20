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

use crate::Entry;
use leo_typed::Type;

use std::collections::HashMap;

/// A abstract data type that tracks the current bindings of identifier names in a Leo program
/// A symbol table has access to all function and circuit names in its parent's symbol table
/// A symbol table cannot access names in a child's symbol table
/// Children cannot access names in another sibling's symbol table
pub struct SymbolTable {
    /// Maps variable name -> (location, type, attributes)
    variables: HashMap<String, Entry>,

    /// Maps function name -> (location, input types, output types, attributes)
    functions: HashMap<String, Entry>,

    /// Maps circuit name -> (location,
    circuits: HashMap<String, Entry>,

    /// The parent of this symbol table
    parent: Option<Box<SymbolTable>>,

    /// The children of this symbol table
    children: Vec<SymbolTable>,
}

impl SymbolTable {
    /// Creates a new symbol table with a given parent symbol table
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            table: HashMap::new(),
            parent,
            children: vec![],
        }
    }

    /// Insert an identifier into the symbol table
    pub fn insert(&mut self, key: String, value: Entry) -> Option<Entry> {
        self.table.insert(key, value)
    }

    /// Adds a pointer to a child symbol table.
    /// Children have access to identifiers in the current symbol table
    pub fn add_child(&mut self, child: SymbolTable) {
        self.children.push(child)
    }
}
