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

use crate::{Expression, ExpressionValue, ResolvedNode, SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Expression {
    pub(crate) fn tuple(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        expressions: Vec<UnresolvedExpression>,
        span: Span,
    ) -> Result<Self, ()> {
        // Expressions should evaluate to expected tuple types
        let expected_element_types = if let Some(type_) = expected_type {
            let types = type_.get_type_tuple(span.clone()).unwrap();

            if expressions.len() != types.len() {
                // TODO: throw tuple dimension mismatch error
                unimplemented!("ERROR: tuple dimensions do not match expected array type")
            }

            types.iter().map(|type_| Some(type_.clone())).collect::<Vec<_>>()
        } else {
            vec![None; expressions.len()]
        };

        // Store actual tuple element type
        let mut actual_element_type = None;
        let mut tuple = vec![];

        // Resolve all tuple elements
        for (expression, element_type) in expressions.into_iter().zip(expected_element_types) {
            let expression_resolved = Expression::resolve(table, (element_type, expression)).unwrap();
            let expression_type = expression_resolved.type_().clone();

            tuple.push(expression_resolved);
            actual_element_type = Some(expression_type)
        }

        // Define tuple type for expression
        let type_ = match actual_element_type {
            Some(type_) => type_,
            None => unimplemented!("ERROR: Tuples of size zero are no-op"),
        };

        Ok(Expression {
            type_,
            value: ExpressionValue::Tuple(tuple, span),
        })
    }
}
