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

use crate::{ResolvedNode, SymbolTable, Type, TypeError};

use leo_typed::{Identifier, Span, Type as UnresolvedType};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionOutputType {
    /// Type of function output
    pub type_: Type,
}

impl ResolvedNode for FunctionOutputType {
    type Error = TypeError;
    /// (function output, span)
    type UnresolvedNode = (Option<UnresolvedType>, Span);

    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, TypeError> {
        let function_output = unresolved.0;
        let span = unresolved.1;

        let type_ = match function_output {
            None => Type::Tuple(vec![]), // functions with no return value return an empty tuple
            Some(type_) => Type::resolve(table, (type_, span))?,
        };

        Ok(FunctionOutputType { type_ })
    }
}

impl FunctionOutputType {
    /// Return a resolved function output type from inside of a circuit
    pub fn from_circuit(
        table: &mut SymbolTable,
        circuit_name: Identifier,
        unresolved: Option<UnresolvedType>,
        span: Span,
    ) -> Result<Self, TypeError> {
        let output_type = match unresolved {
            None => Type::Tuple(vec![]),
            Some(type_) => Type::from_circuit(table, type_, circuit_name, span)?,
        };

        Ok(FunctionOutputType { type_: output_type })
    }
}
