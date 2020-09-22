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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    Add(Expression, Expression),
    Sub(Expression, Expression),
    Mul(Expression, Expression),
    Div(Expression, Expression),
    Pow(Expression, Expression),

    // Boolean operations
    Not(Expression, Span),
    Negate(Expression, Span),
    Or(Expression, Expression, Span),
    And(Expression, Expression, Span),
    Eq(Expression, Expression, Span),
    Ge(Expression, Expression, Span),
    Gt(Expression, Expression, Span),
    Le(Expression, Expression, Span),
    Lt(Expression, Expression, Span),

    // Conditionals
    // (conditional, first_value, second_value, span)
    IfElse(Expression, Expression, Expression, Span),

    // Arrays
    // (array_elements, span)
    Array(Vec<Expression>, Span),
    // (array_name, range, span),
    // ArrayAccess(Expression, RangeOrExpression, Span)

    // Tuples
    // (tuple_elements, span)
    Tuple(Vec<Expression>, Span),
    // (tuple_name, index, span)
    TupleAccess(Expression, usize, Span),

    // Circuits
    // (defined_circuit_name, circuit_members, span)
    // Circuit(Identifier, Vec<CircuitVariableDefinition>, Span),
    // (declared_circuit_name, circuit_member_name, span)
    CircuitMemberAccess(Expression, Identifier, Span),
    // (defined_circuit_name, circuit_static_function_name, span)
    CircuitStaticFunctionAccess(Expression, Identifier, Span),

    // Functions
    // (declared_function_name, function_arguments, span)
    FunctionCall(Expression, Vec<Expression>, Span),
    // (core_function_name, function_arguments, span)
    CoreFunctionCall(String, Vec<Expression>, Span),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Expression {
    /// The type this expression evaluates to
    type_: Type,
    /// The value of this expression
    value: ExpressionValue,
}

impl Expression {
    /// Return the type this expression evaluates to
    pub fn type_(&self) -> &Type {
        &self.type_
    }

    /// Resolve the type of an identifier expression
    fn resolve_identifier(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        identifier: Identifier,
    ) -> Result<Self, ()> {
        // Lookup identifier in symbol table
        let variable = table.get_variable(&identifier.name).unwrap();

        // Get type of symbol table entry
        let variable_type = variable.type_();
        let span = identifier.span.clone();

        // Check the expected type if given
        expected_type.check_type(variable_type, span).unwrap();

        Ok(Expression {
            type_: variable_type.clone(),
            value: ExpressionValue::Identifier(identifier),
        })
    }

    /// Resolve an address expression
    fn resolve_address(expected_type: Option<Type>, address_string: String, span: span) -> Result<Self, ()> {
        let type_ = Type::Address;

        // Check the expected type if given
        expected_type.check_type(&type_, span.clone()).unwrap();

        Ok(Expression {
            type_,
            value: ExpressionValue::Address(address_string, span),
        })
    }

    /// Resolve an boolean expression
    fn resolve_boolean(expected_type: Option<Type>, boolean_string: String, span: span) -> Result<Self, ()> {
        let type_ = Type::Boolean;

        // Check the expected type if given
        expected_type.check_type(&type_, span.clone()).unwrap();

        Ok(Expression {
            type_,
            value: ExpressionValue::Boolean(boolean_string, span),
        })
    }

    /// Resolve an field expression
    fn resolve_field(expected_type: Option<Type>, field_string: String, span: span) -> Result<Self, ()> {
        let type_ = Type::Field;

        // Check the expected type if given
        expected_type.check_type(&type_, span.clone()).unwrap();

        Ok(Expression {
            type_,
            value: ExpressionValue::Field(field_string, span),
        })
    }

    /// Resolve an group expression
    fn resolve_group(expected_type: Option<Type>, group_value: GroupValue) -> Result<Self, ()> {
        let type_ = Type::Group;
        let span = group_value.span();

        // Check the expected type if given
        expected_type.check_type(&type_, span.clone()).unwrap();

        Ok(Expression {
            type_,
            value: ExpressionValue::Group(group_value),
        })
    }

    /// Resolve an implicit expression
    fn resolve_implicit(_expected_type: Option<Type>, _implicit_string: String, _span: span) -> Result<Self, ()> {
        // let type_ = Type::Address;
        //
        // // Check the expected type if given
        // expected_type.check_type(&type_, span.clone()).unwrap();
        //
        // Ok(Expression {
        //     type_,
        //     value: ExpressionValue::Address(implicit_string, span)
        // })

        Err(())
    }

    /// Resolve an integer expression
    fn resolve_integer(
        expected_type: Option<Type>,
        integer_type: IntegerType,
        integer_string: String,
        span: span,
    ) -> Result<Self, ()> {
        let type_ = Type::IntegerType(integer_type);

        // Check the expected type if given
        expected_type.check_type(&type_, span.clone()).unwrap();

        Ok(Expression {
            type_,
            value: ExpressionValue::Address(integer_string, span),
        })
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
            UnresolvedExpression::Identifier(identifier) => Self::resolve_identifier(table, expected_type, identifier),
            UnresolvedExpression::Address(string, span) => Self::resolve_address(expected_type, string, span),
            UnresolvedExpression::Boolean(string, span) => Self::resolve_boolean(expected_type, string, span),
            UnresolvedExpression::Field(string, span) => Self::resolve_field(expected_type, string, span),
            UnresolvedExpression::Group(group_value) => Self::resolve_group(expected_type, group_value),
            UnresolvedExpression::Integer(integer_type, string, span) => {
                Self::resolve_integer(expected_type, integer_type, string, span)
            }

            _ => return Err(()),
        }
    }
}
