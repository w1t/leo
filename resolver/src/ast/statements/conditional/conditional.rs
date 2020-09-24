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

use crate::{Expression, FunctionOutputType, ResolvedNode, Statement, SymbolTable, Type};
use leo_typed::{
    ConditionalNestedOrEndStatement as UnresolvedNestedOrEnd,
    ConditionalStatement as UnresolvedConditional,
    Span,
    Statement as UnresolvedStatement,
};

use serde::{Deserialize, Serialize};

/// A nested `else if` or an ending `else` clause
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionalNestedOrEndStatement {
    Nested(Box<Conditional>),
    End(Vec<Statement>),
}

/// An `if else` statement with resolved inner statements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conditional {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub next: Option<ConditionalNestedOrEndStatement>,
    pub span: Span,
}

impl Conditional {
    pub(crate) fn from_unresolved(
        table: &mut SymbolTable,
        return_type: FunctionOutputType,
        conditional: UnresolvedConditional,
        span: Span,
    ) -> Result<Self, ()> {
        // TODO: Create child symbol table and add variables from parent

        // Resolve the condition to a boolean
        let type_boolean = Some(Type::Boolean);
        let condition_resolved = Expression::resolve(table, (type_boolean, conditional.condition)).unwrap();

        // Resolve all statements
        let statements_resolved = resolve_statements(table, return_type.clone(), conditional.statements).unwrap();

        let next_resolved = conditional.next.map(|next| match next {
            UnresolvedNestedOrEnd::Nested(conditional) => {
                let conditional_resolved =
                    Self::from_unresolved(table, return_type.clone(), *conditional, span.clone()).unwrap();
                ConditionalNestedOrEndStatement::Nested(Box::new(conditional_resolved))
            }
            UnresolvedNestedOrEnd::End(statements) => {
                let statements_resolved = resolve_statements(table, return_type, statements).unwrap();
                ConditionalNestedOrEndStatement::End(statements_resolved)
            }
        });

        Ok(Conditional {
            condition: condition_resolved,
            statements: statements_resolved,
            next: next_resolved,
            span,
        })
    }
}

/// Resolve an array of statements
fn resolve_statements(
    table: &mut SymbolTable,
    return_type: FunctionOutputType,
    statements: Vec<UnresolvedStatement>,
) -> Result<Vec<Statement>, ()> {
    Ok(statements
        .into_iter()
        .map(|statement| Statement::resolve(table, (return_type.clone(), statement)))
        .collect::<Result<Vec<_>, _>>()
        .unwrap())
}

impl Statement {
    /// Resolves a conditional statement
    pub(crate) fn conditional(
        table: &mut SymbolTable,
        return_type: FunctionOutputType,
        conditional: UnresolvedConditional,
        span: Span,
    ) -> Result<Self, ()> {
        let conditional = Conditional::from_unresolved(table, return_type, conditional, span).unwrap();

        Ok(Statement::Conditional(conditional))
    }
}
