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

use crate::{ResolvedNode, SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, GroupValue, Identifier, IntegerType, Span};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressionValue {
    // Identifier
    Identifier(Identifier),

    // Values
    Address(String, Span),
    Boolean(String, Span),
    Field(String, Span),
    Group(GroupValue),
    Integer(IntegerType, String, Span),

    // Number operations
    Add(Box<Expression>, Box<Expression>, Span),
    Sub(Box<Expression>, Box<Expression>, Span),
    Mul(Box<Expression>, Box<Expression>, Span),
    Div(Box<Expression>, Box<Expression>, Span),
    Pow(Box<Expression>, Box<Expression>, Span),
    Negate(Box<Expression>, Span),

    // Boolean operations
    Not(Box<Expression>, Span),
    Or(Box<Expression>, Box<Expression>, Span),
    And(Box<Expression>, Box<Expression>, Span),
    Eq(Box<Expression>, Box<Expression>, Span),
    Ge(Box<Expression>, Box<Expression>, Span),
    Gt(Box<Expression>, Box<Expression>, Span),
    Le(Box<Expression>, Box<Expression>, Span),
    Lt(Box<Expression>, Box<Expression>, Span),

    // Conditionals
    // (conditional, first_value, second_value, span)
    IfElse(Box<Expression>, Box<Expression>, Box<Expression>, Span),

    // Arrays
    // (array_elements, span)
    // Array(Vec<Box<SpreadOrExpression>>, Span),
    // (array_name, range, span)
    // ArrayAccess(Box<Expression>, Box<RangeOrExpression>, Span),

    // Tuples
    // (tuple_elements, span)
    Tuple(Vec<Expression>, Span),
    // (tuple_name, index, span)
    TupleAccess(Box<Expression>, usize, Span),

    // Circuits
    // (defined_circuit_name, circuit_members, span)
    // Circuit(Identifier, Vec<CircuitVariableDefinition>, Span),
    // (declared_circuit name, circuit_member_name, span)
    CircuitMemberAccess(Box<Expression>, Identifier, Span),
    // (defined_circuit name, circuit_static_function_name, span)
    CircuitStaticFunctionAccess(Box<Expression>, Identifier, Span),

    // Functions
    // (declared_function_name, function_arguments, span)
    FunctionCall(Box<Expression>, Vec<Expression>, Span),
    // (core_function_name, function_arguments, span)
    CoreFunctionCall(String, Vec<Expression>, Span),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    /// The type this expression evaluates to
    pub(crate) type_: Type,
    /// The value of this expression
    pub(crate) value: ExpressionValue,
}

impl Expression {
    /// Return the type this expression evaluates to
    pub fn type_(&self) -> &Type {
        &self.type_
    }

    /// Resolve a binary expression from left to right.
    /// If no expected type is given, then the expression resolves to the lhs type.
    pub(crate) fn binary(
        table: &mut SymbolTable,
        mut expected_type: Option<Type>,
        lhs: UnresolvedExpression,
        rhs: UnresolvedExpression,
        _span: Span,
    ) -> Result<(Self, Self), ()> {
        // Resolve lhs with expected type
        let lhs_resolved = Expression::resolve(table, (expected_type, lhs)).unwrap();

        // Set the expected type to the lhs type
        expected_type = Some(lhs_resolved.type_.clone());

        // Resolve the rhs with expected type
        let rhs_resolved = Expression::resolve(table, (expected_type, rhs)).unwrap();

        Ok((lhs_resolved, rhs_resolved))
    }
}

impl ResolvedNode for Expression {
    type Error = ();
    /// (expected type, unresolved expression)
    type UnresolvedNode = (Option<Type>, UnresolvedExpression);

    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let expected_type = unresolved.0;
        let expression = unresolved.1;

        match expression {
            // Identifier
            UnresolvedExpression::Identifier(identifier) => Self::identifier(table, expected_type, identifier),

            // Values
            UnresolvedExpression::Address(string, span) => Self::address(expected_type, string, span),
            UnresolvedExpression::Boolean(string, span) => Self::boolean(expected_type, string, span),
            UnresolvedExpression::Field(string, span) => Self::field(expected_type, string, span),
            UnresolvedExpression::Group(group_value) => Self::group(expected_type, group_value),
            UnresolvedExpression::Implicit(string, span) => Self::implicit(expected_type, string, span),
            UnresolvedExpression::Integer(integer_type, string, span) => {
                Self::integer(expected_type, integer_type, string, span)
            }

            // Number Operations
            UnresolvedExpression::Add(lhs, rhs, span) => Self::add(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Sub(lhs, rhs, span) => Self::sub(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Mul(lhs, rhs, span) => Self::mul(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Div(lhs, rhs, span) => Self::div(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Pow(lhs, rhs, span) => Self::pow(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Negate(expression, span) => Self::negate(table, expected_type, *expression, span),

            _ => return Err(()),
        }
    }
}
