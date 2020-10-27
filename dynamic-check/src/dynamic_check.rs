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

use crate::{DynamicCheckError, Frame, Scope};
use leo_static_check::SymbolTable;
use leo_typed::{Circuit, CircuitMember, Function, Program};

/// Performs a dynamic type inference check over a program.
pub struct DynamicCheck {
    table: SymbolTable,
    frames: Vec<Frame>,
}

impl DynamicCheck {
    ///
    /// Creates a new `DynamicCheck` on a given program and symbol table.
    ///
    /// Evaluates all `TypeAssertion` predicates.
    ///
    pub fn new(program: &Program, symbol_table: SymbolTable) -> Result<(), DynamicCheckError> {
        let mut dynamic_check = Self {
            table: symbol_table,
            frames: Vec::new(),
        };

        dynamic_check.parse_program(program)?;

        dynamic_check.check()
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a program.
    ///
    fn parse_program(&mut self, program: &Program) -> Result<(), DynamicCheckError> {
        // Iterate over circuit types.
        let circuits = program
            .circuits
            .iter()
            .map(|(_identifier, circuit)| circuit)
            .collect::<Vec<_>>();

        // Parse circuit types in program context.
        self.parse_circuits(circuits)?;

        // Iterate over functions.
        let functions = program
            .functions
            .iter()
            .map(|(_identifier, function)| function)
            .collect::<Vec<_>>();

        // Parse functions in program context.
        self.parse_functions(functions)
    }

    ///
    /// Collects a vector of `Frames`s from a vector of circuit functions.
    ///
    fn parse_circuits(&mut self, circuits: Vec<&Circuit>) -> Result<(), DynamicCheckError> {
        for circuit in circuits {
            self.parse_circuit(circuit)?;
        }

        Ok(())
    }

    ///
    /// Collects a vector of `Frames`s from a circuit function.
    ///
    /// Each frame collects a vector of `TypeAssertion` predicates from each function.
    ///
    fn parse_circuit(&mut self, circuit: &Circuit) -> Result<(), DynamicCheckError> {
        let name = &circuit.circuit_name.name;

        // Get circuit type from circuit symbol table.
        let circuit_type = self.table.get_circuit_type(name).unwrap().clone();

        // Create a new function for each circuit member function.
        for circuit_member in &circuit.members {
            // ignore circuit member variables
            if let CircuitMember::CircuitFunction(_, function) = circuit_member {
                // Collect `TypeAssertion` predicates from the function.
                // Pass down circuit self type and circuit variable types to each function.
                let frame = Frame::new_circuit_function(
                    function.to_owned(),
                    circuit_type.clone(),
                    Scope::default(),
                    self.table.clone(),
                )?;

                self.frames.push(frame)
            }
        }

        Ok(())
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of functions.
    ///
    fn parse_functions(&mut self, functions: Vec<&Function>) -> Result<(), DynamicCheckError> {
        for function in functions {
            self.parse_function(function)?;
        }

        Ok(())
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    fn parse_function(&mut self, function: &Function) -> Result<(), DynamicCheckError> {
        let frame = Frame::new_function(function.to_owned(), None, None, self.table.clone())?;

        self.frames.push(frame);

        Ok(())
    }

    ///
    /// Returns the result of evaluating all `TypeAssertion` predicates.
    ///
    /// Will attempt to substitute a `Type` for all `TypeVariable`s.
    /// Returns a `LeoResolvedAst` if all `TypeAssertion` predicates are true.
    /// Returns ERROR if a `TypeAssertion` predicate is false or a solution does not exist.
    ///
    pub fn check(self) -> Result<(), DynamicCheckError> {
        for frame in self.frames {
            frame.check()?;
        }

        Ok(())
    }
}