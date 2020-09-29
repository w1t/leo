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

use crate::{Attribute, ResolvedNode, SymbolTable, Type, TypeError, VariableType};
use leo_typed::{FunctionInputVariable, Identifier, Span};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionInputVariableType {
    /// Name of function input
    pub identifier: Identifier,
    /// Type of function input
    pub type_: Type,
    /// The attributes of the function input
    pub attributes: Vec<Attribute>,
    /// The span of the function input
    pub span: Span,
}

impl ResolvedNode for FunctionInputVariableType {
    type Error = TypeError;
    type UnresolvedNode = FunctionInputVariable;

    /// Type check an input to a function
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let type_ = Type::resolve(table, (unresolved.type_, unresolved.span.clone()))?;
        let attributes = if unresolved.mutable {
            vec![Attribute::Mutable]
        } else {
            vec![]
        };

        Ok(FunctionInputVariableType {
            identifier: unresolved.identifier,
            type_,
            attributes,
            span: unresolved.span,
        })
    }
}

impl FunctionInputVariableType {
    /// Return a resolved function input variable from inside of a circuit
    pub fn from_circuit(
        table: &mut SymbolTable,
        unresolved_function_input: FunctionInputVariable,
        circuit_name: Identifier,
    ) -> Result<Self, TypeError> {
        let type_ = Type::from_circuit(
            table,
            unresolved_function_input.type_,
            circuit_name,
            unresolved_function_input.span.clone(),
        )?;
        let attributes = if unresolved_function_input.mutable {
            vec![Attribute::Mutable]
        } else {
            vec![]
        };

        Ok(FunctionInputVariableType {
            identifier: unresolved_function_input.identifier,
            type_,
            attributes,
            span: unresolved_function_input.span,
        })
    }

    /// Insert this function variable into the given symbol table
    pub fn insert(&self, table: &mut SymbolTable) -> Option<VariableType> {
        let key = self.identifier.name.clone();
        let value = VariableType::from(self.clone());

        table.insert_variable(key, value)
    }
}
