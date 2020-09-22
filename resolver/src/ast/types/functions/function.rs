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
    types::functions::{FunctionInputType, FunctionOutputType},
    SymbolTable,
};
use leo_typed::{Function, Identifier};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionType {
    /// The name of the function definition
    pub identifier: Identifier,
    /// The function inputs
    pub inputs: Vec<FunctionInputType>,
    /// The function return output
    pub output: FunctionOutputType,
}

impl FunctionType {
    /// Resolve a function definition and insert it into the given symbol table
    pub fn insert_definition(table: &mut SymbolTable, unresolved_function: Function) {
        let function_identifier = unresolved_function.identifier;
        let mut inputs = vec![];

        for unresolved_input in unresolved_function.input {
            let input = FunctionInputType::from_unresolved(table, unresolved_input);
            inputs.push(input);
        }

        let output = FunctionOutputType::from_unresolved(table, unresolved_function.returns);

        let function = FunctionType {
            identifier: function_identifier.clone(),
            inputs,
            output,
        };

        table.insert_function(function_identifier, function);
    }

    /// Resolve a circuit function definition and return it {
    pub fn from_circuit(table: &SymbolTable, circuit_name: Identifier, unresolved_function: Function) -> Self {
        let function_identifier = unresolved_function.identifier;
        let mut inputs = vec![];

        for unresolved_input in unresolved_function.input {
            let input = FunctionInputType::from_circuit(table, circuit_name.clone(), unresolved_input);
            inputs.push(input);
        }

        let output = FunctionOutputType::from_circuit(table, circuit_name.clone(), unresolved_function.returns);

        FunctionType {
            identifier: function_identifier.clone(),
            inputs,
            output,
        }
    }
}
