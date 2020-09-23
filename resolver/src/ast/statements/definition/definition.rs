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

use crate::{types::FunctionOutputType, Expression, ResolvedNode, Statement, SymbolTable, Type};
use leo_typed::{
    Declare,
    Expression as UnresolvedExpression,
    Span,
    Statement as UnresolvedStatement,
    VariableName,
    Variables,
};

use serde::{Deserialize, Serialize};

/// A `let` or `const` definition statement.
/// Defines one or more variables with resolved types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Definition {
    pub declare: Declare,
    pub variables: DefinitionVariables,
    pub span: Span,
}

/// One or more variables with resolved types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefinitionVariables {
    Single(VariableName, Expression),
    Tuple(VariableName, Vec<Expression>),
    MultipleVariable(Variables, Expression),
    MultipleVariableTuple(Variables, Vec<Expression>),
}

impl Statement {
    /// Resolves a definition statement
    pub(crate) fn definition(
        table: &mut SymbolTable,
        declare: Declare,
        variables: Variables,
        expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ()> {
        let num_variables = variables.names.len();
        let num_values = expressions.len();

        if num_variables == 1 && num_values == 1 {
            // Define a single variable with a single value
        } else if num_variables == 1 && num_values > 1 {
            // Define a tuple (single variable with multiple values)
        } else if num_variables > 1 && num_values == 1 {
            // Define multiple variables for an expression that returns multiple outputs
        } else {
            // Define multiple variables for multiple expressions
        }

        Ok()
    }

    /// Resolves a single variable with a single value
    fn single(
        table: &mut SymbolTable,
        declare: Declare,
        variable: VariableName,
        expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ()> {
    }

    /// Resolves a tuple (single variable with multiple values)
    fn tuple(
        table: &mut SymbolTable,
        declare: Declare,
        variable: VariableName,
        expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ()> {
    }

    /// Resolves multiple variables for an expression that returns multiple outputs
    fn multiple_variable(
        table: &mut SymbolTable,
        declare: Declare,
        variables: Variables,
        expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ()> {
    }

    /// Resolves multiple variables for multiple expressions
    fn multiple_variable_tuple(
        table: &mut SymbolTable,
        declare: Declare,
        variables: Variables,
        expression: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ()> {
    }
}
