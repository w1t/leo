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
    check_tuple_type,
    Attribute,
    Entry,
    Expression,
    ExpressionValue,
    ResolvedNode,
    Statement,
    SymbolTable,
    Type,
};
use leo_typed::{Declare, Expression as UnresolvedExpression, Span, VariableName, Variables};

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
    Tuple(VariableName, Expression),
    MultipleVariable(Vec<VariableName>, Vec<Expression>),
    MultipleVariableTuple(Vec<VariableName>, Expression),
}

impl DefinitionVariables {
    /// Resolves a single variable with a single value
    fn single(
        table: &mut SymbolTable,
        variable: VariableName,
        expected_type: Option<Type>,
        expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ()> {
        // Resolve expression with given expected type
        let expression_resolved = Expression::resolve(table, (expected_type, expression)).unwrap();
        let type_ = expression_resolved.type_();

        // Insert variable into symbol table
        insert_defined_variable(table, &variable, type_, span.clone()).unwrap();

        Ok(DefinitionVariables::Single(variable, expression_resolved))
    }

    /// Resolves a tuple (single variable with multiple values)
    fn tuple(
        table: &mut SymbolTable,
        variable: VariableName,
        expected_type: Option<Type>,
        expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ()> {
        // Resolve tuple of expressions
        let tuple_resolved = Expression::tuple(table, expected_type, expressions, span.clone()).unwrap();
        let type_ = tuple_resolved.type_();

        // Insert variable into symbol table
        insert_defined_variable(table, &variable, type_, span.clone()).unwrap();

        Ok(DefinitionVariables::Tuple(variable, tuple_resolved))
    }

    /// Resolves multiple variables for multiple expressions
    fn multiple_variable(
        table: &mut SymbolTable,
        variables: Variables,
        expected_type: Option<Type>,
        expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ()> {
        // If the expected type is given, then it must be a tuple of types
        let expected_types = check_tuple_type(expected_type, expressions.len(), span.clone()).unwrap();

        // Check number of variables == types
        if variables.names.len() != expected_types.len() {
            unimplemented!("ERROR: Wrong number of types provided")
        }

        // Check number of variables == expressions
        if variables.names.len() != expressions.len() {
            unimplemented!("ERROR: Wrong number of expressions provided")
        }

        // Resolve expressions
        let mut expressions_resolved = vec![];

        for (expression, type_) in expressions.into_iter().zip(expected_types) {
            let expression_resolved = Expression::resolve(table, (type_, expression)).unwrap();

            expressions_resolved.push(expression_resolved);
        }

        // Insert variables into symbol table
        for (variable, expression) in variables.names.clone().iter().zip(expressions_resolved.iter()) {
            insert_defined_variable(table, variable, expression.type_(), span.clone()).unwrap();
        }

        Ok(DefinitionVariables::MultipleVariable(
            variables.names,
            expressions_resolved,
        ))
    }

    /// Resolves multiple variables for an expression that returns a tuple
    fn multiple_variable_tuple(
        table: &mut SymbolTable,
        variables: Variables,
        expected_type: Option<Type>,
        expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ()> {
        // Resolve tuple expression
        let expression_resolved = Expression::resolve(table, (expected_type, expression)).unwrap();

        let expressions_resolved = match &expression_resolved.value {
            ExpressionValue::Tuple(expressions_resolved, _span) => expressions_resolved.clone(),
            _ => unimplemented!("ERROR: Cannot define multiple variables to tuple expression"),
        };

        // Insert variables into symbol table
        for (variable, expression) in variables.names.clone().iter().zip(expressions_resolved.iter()) {
            insert_defined_variable(table, variable, expression.type_(), span.clone()).unwrap();
        }

        Ok(DefinitionVariables::MultipleVariableTuple(
            variables.names,
            expression_resolved,
        ))
    }
}

/// Inserts a variable definition into the given symbol table
fn insert_defined_variable(
    table: &mut SymbolTable,
    variable: &VariableName,
    type_: &Type,
    _span: Span,
) -> Result<(), ()> {
    let attributes = if variable.mutable {
        vec![Attribute::Mutable]
    } else {
        vec![]
    };

    // Insert variable into symbol table
    let key = variable.identifier.name.clone();
    let value = Entry {
        identifier: variable.identifier.clone(),
        type_: type_.clone(),
        attributes,
    };

    // TODO: Check that variable name was not defined before
    table.insert_variable(key, value);

    Ok(())
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
        let expected_type = variables.type_.clone().map(|type_| Type::from_unresolved(table, type_));

        let variables = if num_variables == 1 && num_values == 1 {
            // Define a single variable with a single value

            DefinitionVariables::single(
                table,
                variables.names[0].clone(),
                expected_type,
                expressions[0].clone(),
                span.clone(),
            )
        } else if num_variables == 1 && num_values > 1 {
            // Define a tuple (single variable with multiple values)

            DefinitionVariables::tuple(
                table,
                variables.names[0].clone(),
                expected_type,
                expressions,
                span.clone(),
            )
        } else if num_variables > 1 && num_values == 1 {
            // Define multiple variables for an expression that returns a tuple

            DefinitionVariables::multiple_variable_tuple(
                table,
                variables,
                expected_type,
                expressions[0].clone(),
                span.clone(),
            )
        } else {
            // Define multiple variables for multiple expressions

            DefinitionVariables::multiple_variable(table, variables, expected_type, expressions, span.clone())
        }?;

        Ok(Statement::Definition(Definition {
            declare,
            variables,
            span,
        }))
    }
}
