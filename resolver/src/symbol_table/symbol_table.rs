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

use crate::{CircuitType, FunctionType, SymbolTableError, VariableType};
use leo_typed::{Identifier, Program as UnresolvedProgram};

use leo_imports::ImportParser;
use std::collections::HashMap;

/// A abstract data type that tracks the current bindings of identifier names in a Leo program.
/// A symbol table has access to all function and circuit names in its parent's symbol table.
/// A symbol table cannot access names in a child's symbol table.
/// Children cannot access names in another sibling's symbol table.
#[derive(Clone)]
pub struct SymbolTable {
    /// Maps variable name -> (location, type, attributes)
    variables: HashMap<String, VariableType>,

    /// Maps circuit name -> (location, variables, functions)
    circuits: HashMap<String, CircuitType>,

    ///Maps function name -> (location, inputs, outputs)
    functions: HashMap<String, FunctionType>,

    /// The parent of this symbol table
    parent: Option<Box<SymbolTable>>,

    /// The children of this symbol table
    children: Vec<SymbolTable>,
}

impl SymbolTable {
    ///
    /// Creates a new symbol table with a given parent symbol table
    ///
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            variables: HashMap::new(),
            circuits: HashMap::new(),
            functions: HashMap::new(),
            parent,
            children: vec![],
        }
    }

    ///
    /// Insert a variable into the symbol table from a given variable name and variable type.
    ///
    /// If the symbol table did not have this name present, `None` is returned.
    ///
    pub fn insert_variable(&mut self, key: String, value: VariableType) -> Option<VariableType> {
        self.variables.insert(key, value)
    }

    ///
    /// Insert a circuit definition into the symbol table from a given circuit name and
    /// circuit type.
    ///
    /// If the symbol table did not have this name present, `None` is returned.
    ///
    pub fn insert_circuit(&mut self, key: Identifier, value: CircuitType) -> Option<CircuitType> {
        self.circuits.insert(key.name, value)
    }

    ///
    /// Insert a function definition into the symbol table from a given function name and
    /// function type.
    ///
    /// If the symbol table did not have this name present, `None` is returned.
    ///
    pub fn insert_function(&mut self, key: Identifier, value: FunctionType) -> Option<FunctionType> {
        self.functions.insert(key.name, value)
    }

    ///
    /// Returns a reference to the
    ///
    ///
    ///
    pub fn get_variable(&self, key: &String) -> Option<&VariableType> {
        match self.variables.get(key) {
            Some(variable) => Some(variable),
            None => {
                // Look in parent symbol table
                match &self.parent {
                    Some(parent) => parent.get_variable(key),
                    None => None,
                }
            }
        }
    }

    ///
    /// Get the current binding of a circuit type.
    ///
    pub fn get_circuit(&self, key: &String) -> Option<&CircuitType> {
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

    ///
    /// Get the current binding of a function type.
    ///
    pub fn get_function(&self, key: &String) -> Option<&FunctionType> {
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

    ///
    /// Adds a child symbol table
    /// Children have access to identifiers in the current symbol table
    ///
    pub fn push_child(&mut self, child: SymbolTable) {
        self.children.push(child)
    }

    ///
    /// Inserts all imported identifiers for a given list of imported programs
    /// No type resolution performed at this step
    ///
    pub fn insert_imports(&mut self, _imports: ImportParser) {}

    ///
    /// Checks for duplicate circuit and function names given a typed syntax tree.
    ///
    /// If a circuit or function name has no collisions, then it is inserted into the symbol table.
    /// Variables defined later in the typed syntax tree cannot have the same name.
    ///
    pub fn pass_one(&mut self, program: &UnresolvedProgram) -> Result<(), SymbolTableError> {
        // Check typed syntax tree circuit names.
        for (identifier, circuit) in program.circuits.iter() {
            // Attempt to insert the circuit name into the symbol table.
            let duplicate = self.insert_variable(identifier.to_string(), VariableType::from(circuit.clone()));

            // Check circuit name is unique.
            if duplicate.is_some() {
                return Err(SymbolTableError::duplicate_circuit(
                    identifier.clone(),
                    circuit.circuit_name.span.clone(),
                ));
            }
        }

        // Check typed syntax tree function names.
        for (identifier, function) in program.functions.iter() {
            // Attempt to insert the function name into the symbol table.
            let duplicate = self.insert_variable(identifier.to_string(), VariableType::from(function.clone()));

            // Check function name is unique.
            if duplicate.is_some() {
                return Err(SymbolTableError::duplicate_function(
                    identifier.clone(),
                    function.span.clone(),
                ));
            }
        }

        Ok(())
    }

    ///
    /// Checks for unknown types in circuit and function definitions given a typed syntax tree.
    ///
    /// If a circuit or function definition only contains known types, then it is inserted into the
    /// symbol table. Variables defined later in the typed syntax tree can lookup the definition and
    /// refer to its expected types.
    ///
    pub fn pass_two(&mut self, program: &UnresolvedProgram) -> Result<(), SymbolTableError> {
        // Check typed syntax tree circuit definitions.
        for (_, unresolved_circuit) in program.circuits.iter() {
            // Attempt to insert the circuit definition into the symbol table.
            CircuitType::insert_definition(self, unresolved_circuit.clone())?;
        }

        // Check typed syntax tree function definitions.
        for (_, unresolved_function) in program.functions.iter() {
            // Attempt to insert the function definition into the symbol table
            FunctionType::insert_definition(self, unresolved_function.clone())?;
        }

        Ok(())
    }
}
