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
use crate::{types::FunctionOutputType, Expression, ResolvedNode, SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span, Statement as UnresolvedStatement};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statement {
    Return(Expression, Span),
    // Definition(Declare, Variables, Vec<Expression>, Span),
    // Assign(Assignee, Expression, Span),
    // Conditional(ConditionalStatement, Span),
    // Iteration(Identifier, Expression, Expression, Vec<Statement>, Span),
    // Console(ConsoleFunctionCall),
    Expression(Expression, Span),
}

impl Statement {
    fn resolve_return(
        table: &mut SymbolTable,
        expected_type: Type,
        expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ()> {
        let expression = Expression::resolve(table, (Some(expected_type), expression)).unwrap();

        Ok(Statement::Return(expression, span))
    }
}

impl ResolvedNode for Statement {
    type Error = ();
    type UnresolvedNode = (FunctionOutputType, UnresolvedStatement);

    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let expected_type = unresolved.0.type_;
        let statement = unresolved.1;

        match statement {
            UnresolvedStatement::Return(expression, span) => {
                Self::resolve_return(table, expected_type, expression, span)
            }
            _ => Err(()),
        }
    }
}
