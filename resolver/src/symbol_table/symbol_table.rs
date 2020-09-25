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

use crate::{CircuitType, Entry, FunctionType, SymbolTableError};
use leo_typed::{Identifier, Program};

use leo_imports::ImportParser;
use std::collections::HashMap;

use std::convert::TryFrom;

/// A abstract data type that tracks the current bindings of identifier names in a Leo program
/// A symbol table has access to all function and circuit names in its parent's symbol table
/// A symbol table cannot access names in a child's symbol table
/// Children cannot access names in another sibling's symbol table
#[derive(Clone)]
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

    /// Insert a circuit definition into the symbol table
    pub fn insert_circuit(&mut self, key: Identifier, value: CircuitType) -> Option<CircuitType> {
        self.circuits.insert(key, value)
    }

    /// Insert a function definition into the symbol table
    pub fn insert_function(&mut self, key: Identifier, value: FunctionType) -> Option<FunctionType> {
        self.functions.insert(key, value)
    }

    /// Get the current binding of an identifier
    pub fn get_variable(&self, key: &String) -> Option<&Entry> {
        self.variables.get(key)
    }

    /// Get the current binding of a circuit type
    pub fn get_circuit(&self, key: &Identifier) -> Option<&CircuitType> {
        match self.circuits.get(key) {
            Some(circuit) => Some(circuit),
            None => {
                // Look in parent symbol table
                match &self.parent {
                    Some(parent) => parent.get_circuit(key),
                    None => None,
                }
            }
        }
    }

    /// Get the current binding of a function type
    pub fn get_function(&self, key: &Identifier) -> Option<&FunctionType> {
        match self.functions.get(key) {
            Some(circuit) => Some(circuit),
            None => {
                // Look in parent symbol table
                match &self.parent {
                    Some(parent) => parent.get_function(key),
                    None => None,
                }
            }
        }
    }

    /// Adds a child symbol table
    /// Children have access to identifiers in the current symbol table
    pub fn push_child(&mut self, child: SymbolTable) {
        self.children.push(child)
    }

    /// Inserts all imported identifiers for a given list of imported programs
    /// No type resolution performed at this step
    pub fn insert_imports(&mut self, _imports: ImportParser) {}

    /// Inserts all circuits and functions as variable types
    /// No type resolution performed at this step
    /// `let f = Foo { }; // f has type circuit Foo`
    pub fn insert_program_variables(&mut self, program: &Program) -> Result<(), SymbolTableError> {
        // insert program circuits
        for (identifier, circuit) in program.circuits.iter() {
            let duplicate = self.insert_variable(identifier.to_string(), Entry::from(circuit.clone()));
            if duplicate.is_some() {
                return Err(SymbolTableError::duplicate_circuit(
                    identifier.clone(),
                    circuit.circuit_name.span.clone(),
                ));
            }
        }

        // insert program functions
        for (identifier, function) in program.functions.iter() {
            let duplicate = self.insert_variable(identifier.to_string(), Entry::from(function.clone()));
            if duplicate.is_some() {
                return Err(SymbolTableError::duplicate_function(
                    identifier.clone(),
                    function.span.clone(),
                ));
            }
        }

        Ok(())
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
}
