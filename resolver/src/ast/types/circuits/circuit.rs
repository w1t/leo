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

use crate::{
    types::circuits::{CircuitFunctionType, CircuitVariableType},
    Attribute,
    FunctionType,
    SymbolTable,
    Type,
    TypeError,
};
use leo_typed::{Circuit, CircuitMember, Identifier};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircuitType {
    /// The name of the circuit definition
    pub identifier: Identifier,
    /// The circuit member variables
    pub variables: Vec<CircuitVariableType>,
    /// The circuit member functions
    pub functions: Vec<CircuitFunctionType>,
}

impl CircuitType {
    /// Resolve a circuit definition and insert it into the given symbol table.
    pub fn insert_definition(table: &mut SymbolTable, unresolved_circuit: Circuit) -> Result<(), TypeError> {
        let circuit_identifier = unresolved_circuit.circuit_name;
        let mut variables = vec![];
        let mut functions = vec![];

        for member in unresolved_circuit.members {
            match member {
                CircuitMember::CircuitVariable(mutable, variable_identifier, type_) => {
                    // Resolve the type of the circuit member variable
                    let type_ = Type::from_circuit(
                        table,
                        type_,
                        circuit_identifier.clone(),
                        circuit_identifier.span.clone(),
                    )?;

                    let attributes = if mutable { vec![Attribute::Mutable] } else { vec![] };

                    let variable = CircuitVariableType {
                        identifier: variable_identifier,
                        type_,
                        attributes,
                    };

                    variables.push(variable);
                }
                CircuitMember::CircuitFunction(is_static, function) => {
                    let function_type = FunctionType::from_circuit(table, circuit_identifier.clone(), function)?;
                    let attributes = if is_static { vec![Attribute::Static] } else { vec![] };

                    let function = CircuitFunctionType {
                        function: function_type,
                        attributes,
                    };

                    functions.push(function);
                }
            }
        }

        let circuit = CircuitType {
            identifier: circuit_identifier.clone(),
            variables,
            functions,
        };

        table.insert_circuit(circuit_identifier, circuit);

        Ok(())
    }

    /// Resolves the type of a circuit variable or the return type of a circuit function
    pub fn member_type(&self, identifier: &Identifier) -> Result<&Type, ()> {
        let matched_variable = self
            .variables
            .iter()
            .find(|variable| variable.identifier.eq(identifier));

        match matched_variable {
            Some(variable) => Ok(&variable.type_),
            None => {
                let matched_function = self
                    .functions
                    .iter()
                    .find(|function| function.function.identifier.eq(identifier));
                match matched_function {
                    Some(function) => Ok(&function.function.output.type_),
                    None => Err(()),
                }
            }
        }
    }
}
