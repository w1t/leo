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

use crate::{CircuitType, Entry, FunctionType};
use leo_typed::{Identifier, Program};

use leo_imports::ImportParser;
use std::collections::HashMap;

use std::convert::TryFrom;

/// A abstract data type that tracks the current bindings of identifier names in a Leo program
/// A symbol table has access to all function and circuit names in its parent's symbol table
/// A symbol table cannot access names in a child's symbol table
/// Children cannot access names in another sibling's symbol table
pub struct SymbolTable {
    /// Maps variable name -> (location, type, attributes)
    variables: HashMap<String, Entry>,

    /// Maps circuit name -> (location, inputs, outputs)
    circuits: HashMap<Identifier, CircuitType>,

    ///Maps function name -> (location, variables, functions)
    functions: HashMap<Identifier, FunctionType>,

    /// The parent of this symbol table
    parent: Option<Box<SymbolTable>>,

    /// The children of this symbol table
    children: Vec<SymbolTable>,
}

impl SymbolTable {
    /// Creates a new symbol table with a given parent symbol table
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            variables: HashMap::new(),
            circuits: HashMap::new(),
            functions: HashMap::new(),
            parent,
            children: vec![],
        }
    }

    /// Insert an identifier into the symbol table
    pub fn insert_variable(&mut self, key: String, value: Entry) -> Option<Entry> {
        self.variables.insert(key, value)
    }

    /// Get the current binding of an identifier
    pub fn get_variable(&self, key: &String) -> Option<&Entry> {
        self.variables.get(key)
    }

    /// Insert a circuit definition into the symbol table
    pub fn insert_circuit(&mut self, key: Identifier, value: CircuitType) -> Option<CircuitType> {
        self.circuits.insert(key, value)
    }

    /// Insert a function definition into the symbol table
    pub fn insert_function(&mut self, key: Identifier, value: FunctionType) -> Option<FunctionType> {
        self.functions.insert(key, value)
    }

    /// Adds a pointer to a child symbol table
    /// Children have access to identifiers in the current symbol table
    pub fn add_child(&mut self, child: SymbolTable) {
        self.children.push(child)
    }

    /// Inserts all imported identifiers for a given list of imported programs
    /// No type resolution performed at this step
    pub fn insert_imports(&mut self, _imports: ImportParser) {}

    /// Inserts all circuits and functions as variable types
    /// `let f = Foo { }; // f has type circuit Foo`
    pub fn insert_variables(&mut self, program: &Program) {
        // insert program circuits
        program.circuits.iter().for_each(|(identifier, circuit)| {
            let duplicate = self.insert_variable(identifier.to_string(), Entry::try_from(circuit.clone()).unwrap());
            // TODO: throw error for duplicate circuit names
            if duplicate.is_some() {
                unimplemented!("ERROR: duplicate circuit definition `{}`", duplicate.unwrap());
            }
        });

        // insert program functions
        program.functions.iter().for_each(|(identifier, function)| {
            let duplicate = self.insert_variable(identifier.to_string(), Entry::try_from(function.clone()).unwrap());
            // TODO: throw error for duplicate function names
            if duplicate.is_some() {
                unimplemented!("ERROR: duplicate function definition `{}`", duplicate.unwrap());
            }
        });
    }

    /// Inserts all circuits and functions as their respective types with additional information
    /// Type resolution for circuit and function signatures completed during this step
    pub fn insert_definitions(&mut self, program: &Program) {
        // insert program circuit types
        program.circuits.iter().for_each(|(_identifier, unresolved_circuit)| {
            CircuitType::insert_definition(self, unresolved_circuit.clone());
        });

        // insert program function types
        program.functions.iter().for_each(|(_identifier, unresolved_function)| {
            FunctionType::insert_definition(self, unresolved_function.clone())
        });
    }

    /// Inserts all function and circuit identifiers for a given program
    /// No type resolution performed at this step
    pub fn insert_program(&mut self, program: &Program) {
        // Pass 1: Insert circuits and functions as types
        self.insert_variables(program);

        // Pass 2: Insert circuits and functions as definitions
        self.insert_definitions(program);
    }
}
