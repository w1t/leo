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

//! Enforces array access in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_ast::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_array_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expected_type: Option<Type>,
        array: Expression,
        index: Expression,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let array = match self.enforce_operand(cs, file_scope, function_scope, expected_type, array, span)? {
            ConstrainedValue::Array(array) => array,
            value => return Err(ExpressionError::undefined_array(value.to_string(), span.to_owned())),
        };

        let index_resolved = self.enforce_index(cs, file_scope, function_scope, index, span)?;
        Ok(array[index_resolved].to_owned())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_array_range_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expected_type: Option<Type>,
        array: Expression,
        left: Option<Expression>,
        right: Option<Expression>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let array = match self.enforce_operand(cs, file_scope, function_scope, expected_type, array, span)? {
            ConstrainedValue::Array(array) => array,
            value => return Err(ExpressionError::undefined_array(value.to_string(), span.to_owned())),
        };

        let from_resolved = match left {
            Some(from_index) => self.enforce_index(cs, file_scope, function_scope, from_index, span)?,
            None => 0usize, // Array slice starts at index 0
        };
        let to_resolved = match right {
            Some(to_index) => self.enforce_index(cs, file_scope, function_scope, to_index, span)?,
            None => array.len(), // Array slice ends at array length
        };
        Ok(ConstrainedValue::Array(array[from_resolved..to_resolved].to_owned()))
    }
}
