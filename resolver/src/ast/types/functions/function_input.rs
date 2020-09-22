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

use crate::{Attribute, SymbolTable, Type};
use leo_typed::{FunctionInput, FunctionInputVariable, Identifier};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionInputVariableType {
    /// Name of function input
    pub identifier: Identifier,
    /// Type of function input
    pub type_: Type,
    /// The attributes of the function variable
    pub attributes: Vec<Attribute>,
}

impl FunctionInputVariableType {
    /// Return a resolved function input variable type given an unresolved function input variable
    pub fn from_unresolved(table: &SymbolTable, unresolved_function_input: FunctionInputVariable) -> Self {
        let identifier = unresolved_function_input.identifier;
        let type_ = Type::from_unresolved(table, unresolved_function_input.type_);
        let attributes = if unresolved_function_input.mutable {
            vec![Attribute::Mutable]
        } else {
            vec![]
        };

        FunctionInputVariableType {
            identifier,
            type_,
            attributes,
        }
    }

    /// Return a resolved function input variable from inside of a circuit
    pub fn from_circuit(
        table: &SymbolTable,
        circuit_name: Identifier,
        unresolved_function_input: FunctionInputVariable,
    ) -> Self {
        let identifier = unresolved_function_input.identifier;
        let type_ = Type::from_circuit(table, circuit_name, unresolved_function_input.type_);
        let attributes = if unresolved_function_input.mutable {
            vec![Attribute::Mutable]
        } else {
            vec![]
        };

        FunctionInputVariableType {
            identifier,
            type_,
            attributes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FunctionInputType {
    InputKeyword(Identifier),
    Variable(FunctionInputVariableType),
}

impl FunctionInputType {
    /// Return a resolved function input given an unresolved function input
    pub fn from_unresolved(table: &SymbolTable, unresolved_input_variable: FunctionInput) -> Self {
        match unresolved_input_variable {
            FunctionInput::InputKeyword(identifier) => FunctionInputType::InputKeyword(identifier),
            FunctionInput::Variable(unresolved_function_input) => {
                let function_input = FunctionInputVariableType::from_unresolved(table, unresolved_function_input);

                FunctionInputType::Variable(function_input)
            }
        }
    }

    /// Return a resolved function input from inside of a circuit
    pub fn from_circuit(table: &SymbolTable, circuit_name: Identifier, unresolved_input: FunctionInput) -> Self {
        match unresolved_input {
            FunctionInput::InputKeyword(identifier) => FunctionInputType::InputKeyword(identifier),
            FunctionInput::Variable(unresolved_function_input) => {
                let function_input =
                    FunctionInputVariableType::from_circuit(table, circuit_name, unresolved_function_input);

                FunctionInputType::Variable(function_input)
            }
        }
    }
}
