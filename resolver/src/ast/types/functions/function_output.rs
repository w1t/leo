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

use crate::{SymbolTable, Type};

use leo_typed::{Identifier, Type as UnresolvedType};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionOutputType {
    /// Type of function output
    pub type_: Type,
}

impl FunctionOutputType {
    /// Return a resolved function output type given an unresolved function output
    pub fn from_unresolved(table: &SymbolTable, unresolved: Option<UnresolvedType>) -> Self {
        let output_type = match unresolved {
            None => Type::Tuple(vec![]), // functions with no return value return an empty tuple,
            Some(type_) => Type::from_unresolved(table, type_),
        };

        FunctionOutputType { type_: output_type }
    }

    /// Return a resolved function output type from inside of a circuit
    pub fn from_circuit(table: &SymbolTable, circuit_name: Identifier, unresolved: Option<UnresolvedType>) -> Self {
        let output_type = match unresolved {
            None => Type::Tuple(vec![]),
            Some(type_) => Type::from_circuit(table, circuit_name, type_),
        };

        FunctionOutputType { type_: output_type }
    }
}
