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
    ast::expressions::array::SpreadOrExpression,
    Expression,
    ExpressionValue,
    ResolvedNode,
    SymbolTable,
    Type,
};
use leo_typed::{Span, SpreadOrExpression as UnresolvedSpreadOrExpression};

impl Expression {
    /// Resolves an array expression
    pub(crate) fn array(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        expressions: Vec<UnresolvedSpreadOrExpression>,
        span: Span,
    ) -> Result<Self, ()> {
        let mut array = vec![];
        // Expressions should evaluate to array or value with expected type

        for expression in expressions {
            let expression_resolved = SpreadOrExpression::resolve(table, (expected_type.clone(), expression)).unwrap();
            array.push(Box::new(expression_resolved));
        }

        Ok(Expression {
            type_: expression_resolved.type_().clone(),
            value: ExpressionValue::Array(array, span),
        })
    }
}
