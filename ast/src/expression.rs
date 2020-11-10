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
    ArrayDimensions,
    CircuitVariableDefinition,
    GroupValue,
    Identifier,
    IntegerType,
    PositiveNumber,
    RangeOrExpression,
    Span,
    SpreadOrExpression,
};
use leo_grammar::{
    access::{Access, AssigneeAccess},
    common::{Assignee, Identifier as AstIdentifier},
    expressions::{
        ArrayInitializerExpression,
        ArrayInlineExpression,
        BinaryExpression,
        CircuitInlineExpression,
        Expression as AstExpression,
        PostfixExpression,
        TernaryExpression,
        UnaryExpression,
    },
    operations::{BinaryOperation, UnaryOperation},
    values::{
        AddressValue,
        BooleanValue,
        FieldValue,
        GroupValue as AstGroupValue,
        IntegerValue,
        NumberValue as AstNumber,
        Value,
    },
};

use leo_grammar::{access::TupleAccess, expressions::TupleExpression};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    // Identifier
    Identifier(Identifier),

    // Values
    Address(String, Span),
    Boolean(String, Span),
    Field(String, Span),
    Group(Box<GroupValue>),
    Implicit(String, Span),
    Integer(IntegerType, String, Span),

    // Number operations
    Add(Box<(Expression, Expression)>, Span),
    Sub(Box<(Expression, Expression)>, Span),
    Mul(Box<(Expression, Expression)>, Span),
    Div(Box<(Expression, Expression)>, Span),
    Pow(Box<(Expression, Expression)>, Span),

    // Boolean operations
    Not(Box<Expression>, Span),
    Negate(Box<Expression>, Span),
    Or(Box<(Expression, Expression)>, Span),
    And(Box<(Expression, Expression)>, Span),
    Eq(Box<(Expression, Expression)>, Span),
    Ge(Box<(Expression, Expression)>, Span),
    Gt(Box<(Expression, Expression)>, Span),
    Le(Box<(Expression, Expression)>, Span),
    Lt(Box<(Expression, Expression)>, Span),

    // Conditionals
    // (conditional, first_value, second_value, span)
    IfElse(Box<(Expression, Expression, Expression)>, Span),

    // Arrays
    // (array_elements, span)
    ArrayInline(Vec<SpreadOrExpression>, Span),
    // ((array element, dimensions), span)
    ArrayInitializer(Box<(Expression, ArrayDimensions)>, Span),
    // ((array_name, range), span)
    ArrayAccess(Box<(Expression, RangeOrExpression)>, Span),

    // Tuples
    // (tuple_elements, span)
    Tuple(Vec<Expression>, Span),
    // ((tuple_name, index), span)
    TupleAccess(Box<(Expression, PositiveNumber)>, Span),

    // Circuits
    // (defined_circuit_name, circuit_members, span)
    Circuit(Identifier, Vec<CircuitVariableDefinition>, Span),
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

impl Expression {
    pub fn set_span(&mut self, new_span: Span) {
        match self {
            Expression::Field(_, old_span) => *old_span = new_span,
            Expression::Group(value) => value.set_span(new_span),

            Expression::Add(_, old_span) => *old_span = new_span,
            Expression::Sub(_, old_span) => *old_span = new_span,
            Expression::Mul(_, old_span) => *old_span = new_span,
            Expression::Div(_, old_span) => *old_span = new_span,
            Expression::Pow(_, old_span) => *old_span = new_span,

            Expression::Not(_, old_span) => *old_span = new_span,
            Expression::Or(_, old_span) => *old_span = new_span,
            Expression::And(_, old_span) => *old_span = new_span,
            Expression::Eq(_, old_span) => *old_span = new_span,
            Expression::Ge(_, old_span) => *old_span = new_span,
            Expression::Gt(_, old_span) => *old_span = new_span,
            Expression::Le(_, old_span) => *old_span = new_span,
            Expression::Lt(_, old_span) => *old_span = new_span,

            Expression::IfElse(_, old_span) => *old_span = new_span,
            Expression::ArrayInline(_, old_span) => *old_span = new_span,
            Expression::ArrayInitializer(_, old_span) => *old_span = new_span,
            Expression::ArrayAccess(_, old_span) => *old_span = new_span,

            Expression::Tuple(_, old_span) => *old_span = new_span,
            Expression::TupleAccess(_, old_span) => *old_span = new_span,

            Expression::Circuit(_, _, old_span) => *old_span = new_span,
            Expression::CircuitMemberAccess(_, _, old_span) => *old_span = new_span,
            Expression::CircuitStaticFunctionAccess(_, _, old_span) => *old_span = new_span,

            Expression::FunctionCall(_, _, old_span) => *old_span = new_span,
            Expression::CoreFunctionCall(_, _, old_span) => *old_span = new_span,
            _ => {}
        }
    }
}

impl<'ast> fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Variables
            Expression::Identifier(ref variable) => write!(f, "{}", variable),

            // Values
            Expression::Address(ref address, ref _span) => write!(f, "{}", address),
            Expression::Boolean(ref bool, ref _span) => write!(f, "{}", bool),
            Expression::Field(ref field, ref _span) => write!(f, "{}", field),
            Expression::Group(ref group) => write!(f, "{}", group),
            Expression::Implicit(ref value, ref _span) => write!(f, "{}", value),
            Expression::Integer(ref type_, ref integer, ref _span) => write!(f, "{}{}", integer, type_),

            // Number operations
            Expression::Negate(ref expression, ref _span) => write!(f, "-{}", expression),
            Expression::Add(ref left_right, ref _span) => write!(f, "{} + {}", left_right.0, left_right.1),
            Expression::Sub(ref left_right, ref _span) => write!(f, "{} - {}", left_right.0, left_right.1),
            Expression::Mul(ref left_right, ref _span) => write!(f, "{} * {}", left_right.0, left_right.1),
            Expression::Div(ref left_right, ref _span) => write!(f, "{} / {}", left_right.0, left_right.1),
            Expression::Pow(ref left_right, ref _span) => write!(f, "{} ** {}", left_right.0, left_right.1),

            // Boolean operations
            Expression::Not(ref expression, ref _span) => write!(f, "!{}", expression),
            Expression::Or(ref lhs_rhs, ref _span) => write!(f, "{} || {}", lhs_rhs.0, lhs_rhs.1),
            Expression::And(ref lhs_rhs, ref _span) => write!(f, "{} && {}", lhs_rhs.0, lhs_rhs.1),
            Expression::Eq(ref lhs_rhs, ref _span) => write!(f, "{} == {}", lhs_rhs.0, lhs_rhs.1),
            Expression::Ge(ref lhs_rhs, ref _span) => write!(f, "{} >= {}", lhs_rhs.0, lhs_rhs.1),
            Expression::Gt(ref lhs_rhs, ref _span) => write!(f, "{} > {}", lhs_rhs.0, lhs_rhs.1),
            Expression::Le(ref lhs_rhs, ref _span) => write!(f, "{} <= {}", lhs_rhs.0, lhs_rhs.1),
            Expression::Lt(ref lhs_rhs, ref _span) => write!(f, "{} < {}", lhs_rhs.0, lhs_rhs.1),

            // Conditionals
            Expression::IfElse(ref triplet, ref _span) => {
                write!(f, "if {} then {} else {} fi", triplet.0, triplet.1, triplet.2)
            }

            // Arrays
            Expression::ArrayInline(ref array, ref _span) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Expression::ArrayInitializer(ref array, ref _span) => write!(f, "[{}; {}]", array.0, array.1),
            Expression::ArrayAccess(ref array_w_index, ref _span) => {
                write!(f, "{}[{}]", array_w_index.0, array_w_index.1)
            }

            // Tuples
            Expression::Tuple(ref tuple, ref _span) => {
                let values = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "({})", values)
            }
            Expression::TupleAccess(ref tuple_w_index, ref _span) => {
                write!(f, "{}.{}", tuple_w_index.0, tuple_w_index.1)
            }

            // Circuits
            Expression::Circuit(ref var, ref members, ref _span) => {
                write!(f, "{} {{", var)?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.identifier, member.expression)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            Expression::CircuitMemberAccess(ref circuit_name, ref member, ref _span) => {
                write!(f, "{}.{}", circuit_name, member)
            }
            Expression::CircuitStaticFunctionAccess(ref circuit_name, ref member, ref _span) => {
                write!(f, "{}::{}", circuit_name, member)
            }

            // Function calls
            Expression::FunctionCall(ref function, ref arguments, ref _span) => {
                write!(f, "{}(", function,)?;
                for (i, param) in arguments.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if i < arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expression::CoreFunctionCall(ref function, ref arguments, ref _span) => {
                write!(f, "{}(", function,)?;
                for (i, param) in arguments.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if i < arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl<'ast> From<CircuitInlineExpression<'ast>> for Expression {
    fn from(expression: CircuitInlineExpression<'ast>) -> Self {
        let circuit_name = Identifier::from(expression.name);
        let members = expression
            .members
            .into_iter()
            .map(CircuitVariableDefinition::from)
            .collect::<Vec<CircuitVariableDefinition>>();

        Expression::Circuit(circuit_name, members, Span::from(expression.span))
    }
}

impl<'ast> From<PostfixExpression<'ast>> for Expression {
    fn from(expression: PostfixExpression<'ast>) -> Self {
        let variable = Expression::Identifier(Identifier::from(expression.name));

        // ast::PostFixExpression contains an array of "accesses": `a(34)[42]` is represented as `[a, [Call(34), Select(42)]]`, but Access call expressions
        // are recursive, so it is `Select(Call(a, 34), 42)`. We apply this transformation here

        // we start with the id, and we fold the array of accesses by wrapping the current value
        expression
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                // Handle array accesses
                Access::Array(array) => Expression::ArrayAccess(
                    Box::new((acc, RangeOrExpression::from(array.expression))),
                    Span::from(array.span),
                ),

                // Handle tuple access
                Access::Tuple(tuple) => Expression::TupleAccess(
                    Box::new((acc, PositiveNumber::from(tuple.number))),
                    Span::from(tuple.span),
                ),

                // Handle function calls
                Access::Call(function) => {
                    let span = Span::from(function.span);
                    Expression::FunctionCall(
                        Box::new(acc),
                        function.expressions.into_iter().map(Expression::from).collect(),
                        span,
                    )
                }

                // Handle circuit member accesses
                Access::Object(circuit_object) => Expression::CircuitMemberAccess(
                    Box::new(acc),
                    Identifier::from(circuit_object.identifier),
                    Span::from(circuit_object.span),
                ),
                Access::StaticObject(circuit_object) => Expression::CircuitStaticFunctionAccess(
                    Box::new(acc),
                    Identifier::from(circuit_object.identifier),
                    Span::from(circuit_object.span),
                ),
            })
    }
}

impl<'ast> From<AstExpression<'ast>> for Expression {
    fn from(expression: AstExpression<'ast>) -> Self {
        match expression {
            AstExpression::Value(value) => Expression::from(value),
            AstExpression::Identifier(variable) => Expression::from(variable),
            AstExpression::Unary(expression) => Expression::from(*expression),
            AstExpression::Binary(expression) => Expression::from(*expression),
            AstExpression::Ternary(expression) => Expression::from(*expression),
            AstExpression::ArrayInline(expression) => Expression::from(expression),
            AstExpression::ArrayInitializer(expression) => Expression::from(*expression),
            AstExpression::Tuple(expression) => Expression::from(expression),
            AstExpression::CircuitInline(expression) => Expression::from(expression),
            AstExpression::Postfix(expression) => Expression::from(expression),
        }
    }
}

// Assignee -> Expression for operator assign statements
impl<'ast> From<Assignee<'ast>> for Expression {
    fn from(assignee: Assignee<'ast>) -> Self {
        let variable = Expression::Identifier(Identifier::from(assignee.name));

        // we start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                AssigneeAccess::Member(circuit_member) => Expression::CircuitMemberAccess(
                    Box::new(acc),
                    Identifier::from(circuit_member.identifier),
                    Span::from(circuit_member.span),
                ),
                AssigneeAccess::Array(array) => Expression::ArrayAccess(
                    Box::new((acc, RangeOrExpression::from(array.expression))),
                    Span::from(array.span),
                ),
                AssigneeAccess::Tuple(tuple) => Expression::TupleAccess(
                    Box::new((acc, PositiveNumber::from(tuple.number))),
                    Span::from(tuple.span.clone()),
                ),
            })
    }
}

impl<'ast> From<BinaryExpression<'ast>> for Expression {
    fn from(expression: BinaryExpression<'ast>) -> Self {
        match expression.operation {
            // Boolean operations
            BinaryOperation::Or => Expression::Or(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::And => Expression::And(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Eq => Expression::Eq(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Ne => {
                let span = Span::from(expression.span);
                let negated = Expression::Eq(
                    Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                    span.clone(),
                );

                Expression::Not(Box::new(negated), span)
            }
            BinaryOperation::Ge => Expression::Ge(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Gt => Expression::Gt(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Le => Expression::Le(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Lt => Expression::Lt(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            // Number operations
            BinaryOperation::Add => Expression::Add(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Sub => Expression::Sub(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Mul => Expression::Mul(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Div => Expression::Div(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
            BinaryOperation::Pow => Expression::Pow(
                Box::new((Expression::from(expression.left), Expression::from(expression.right))),
                Span::from(expression.span),
            ),
        }
    }
}

impl<'ast> From<TernaryExpression<'ast>> for Expression {
    fn from(expression: TernaryExpression<'ast>) -> Self {
        Expression::IfElse(
            Box::new((
                Expression::from(expression.first),
                Expression::from(expression.second),
                Expression::from(expression.third),
            )),
            Span::from(expression.span),
        )
    }
}

impl<'ast> From<ArrayInlineExpression<'ast>> for Expression {
    fn from(array: ArrayInlineExpression<'ast>) -> Self {
        Expression::ArrayInline(
            array.expressions.into_iter().map(SpreadOrExpression::from).collect(),
            Span::from(array.span),
        )
    }
}

impl<'ast> From<ArrayInitializerExpression<'ast>> for Expression {
    fn from(array: ArrayInitializerExpression<'ast>) -> Self {
        Expression::ArrayInitializer(
            Box::new((
                Expression::from(array.expression),
                ArrayDimensions::from(array.dimensions),
            )),
            Span::from(array.span),
        )
    }
}

impl<'ast> From<TupleExpression<'ast>> for Expression {
    fn from(tuple: TupleExpression<'ast>) -> Self {
        Expression::Tuple(
            tuple.expressions.into_iter().map(Expression::from).collect(),
            Span::from(tuple.span),
        )
    }
}

impl<'ast> From<Value<'ast>> for Expression {
    fn from(value: Value<'ast>) -> Self {
        match value {
            Value::Address(address) => Expression::from(address),
            Value::Boolean(boolean) => Expression::from(boolean),
            Value::Field(field) => Expression::from(field),
            Value::Group(group) => Expression::from(group),
            Value::Implicit(number) => Expression::from(number),
            Value::Integer(integer) => Expression::from(integer),
        }
    }
}

impl<'ast> From<UnaryExpression<'ast>> for Expression {
    fn from(expression: UnaryExpression<'ast>) -> Self {
        match expression.operation {
            UnaryOperation::Not(_) => Expression::Not(
                Box::new(Expression::from(expression.expression)),
                Span::from(expression.span),
            ),
            UnaryOperation::Negate(_) => Expression::Negate(
                Box::new(Expression::from(expression.expression)),
                Span::from(expression.span),
            ),
        }
    }
}

impl<'ast> From<AddressValue<'ast>> for Expression {
    fn from(address: AddressValue<'ast>) -> Self {
        Expression::Address(address.address.value, Span::from(address.span))
    }
}

impl<'ast> From<BooleanValue<'ast>> for Expression {
    fn from(boolean: BooleanValue<'ast>) -> Self {
        Expression::Boolean(boolean.value, Span::from(boolean.span))
    }
}

impl<'ast> From<FieldValue<'ast>> for Expression {
    fn from(field: FieldValue<'ast>) -> Self {
        Expression::Field(field.number.to_string(), Span::from(field.span))
    }
}

impl<'ast> From<AstGroupValue<'ast>> for Expression {
    fn from(ast_group: AstGroupValue<'ast>) -> Self {
        Expression::Group(Box::new(GroupValue::from(ast_group)))
    }
}

impl<'ast> From<AstNumber<'ast>> for Expression {
    fn from(number: AstNumber<'ast>) -> Self {
        let (value, span) = match number {
            AstNumber::Positive(number) => (number.value, number.span),
            AstNumber::Negative(number) => (number.value, number.span),
        };

        Expression::Implicit(value, Span::from(span))
    }
}

impl<'ast> From<IntegerValue<'ast>> for Expression {
    fn from(integer: IntegerValue<'ast>) -> Self {
        let span = Span::from(integer.span().clone());
        let (type_, value) = match integer {
            IntegerValue::Signed(integer) => {
                let type_ = IntegerType::from(integer.type_);
                let number = match integer.number {
                    AstNumber::Negative(number) => number.value,
                    AstNumber::Positive(number) => number.value,
                };

                (type_, number)
            }
            IntegerValue::Unsigned(integer) => {
                let type_ = IntegerType::from(integer.type_);
                let number = integer.number.value;

                (type_, number)
            }
        };

        Expression::Integer(type_, value, span)
    }
}

impl<'ast> From<TupleAccess<'ast>> for Expression {
    fn from(tuple: TupleAccess<'ast>) -> Self {
        Expression::Implicit(tuple.number.to_string(), Span::from(tuple.span))
    }
}

impl<'ast> From<AstIdentifier<'ast>> for Expression {
    fn from(identifier: AstIdentifier<'ast>) -> Self {
        Expression::Identifier(Identifier::from(identifier))
    }
}