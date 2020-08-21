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

//! Enforces that one return value is produced in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};

use leo_typed::{Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// iterates through a vector of results and selects one based off of indicators
    pub fn conditionally_select_result<CS: ConstraintSystem<F>>(
        cs: &mut CS,
        expected_return: Option<Type>,
        results: Vec<(Boolean, ConstrainedValue<F, G>)>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Initialize empty return value
        let mut return_value = ConstrainedValue::Tuple(vec![]);

        // if the function does not expect a return type, make sure there are no returned results.
        let return_type = match expected_return {
            Some(return_type) => return_type,
            None => {
                if results.is_empty() {
                    return Ok(return_value);
                } else {
                    return Err(StatementError::invalid_number_of_returns(0, results.len(), span));
                }
            }
        };

        // Error if the function or one of its branches does not return
        if let None = results
            .iter()
            .find(|(indicator, _res)| indicator.eq(&Boolean::constant(true)))
        {
            return Err(StatementError::no_returns(return_type, span));
        }

        // Find the return value
        let mut ignored = vec![];
        let mut found_return = false;
        for (indicator, result) in results.into_iter() {
            // Error if a statement returned a result with an incorrect type
            let result_type = result.to_type(span.clone())?;
            if return_type != result_type {
                return Err(StatementError::arguments_type(&return_type, &result_type, span));
            }

            if indicator.eq(&Boolean::Constant(true)) {
                // Error if we already have a return value
                if found_return {
                    return Err(StatementError::multiple_returns(span));
                } else {
                    return_value = result;
                    found_return = true;
                }
            } else {
                ignored.push((indicator, result));
            }
        }

        // Conditionally select out the ignored results.
        for (i, (indicator, result)) in ignored.into_iter().enumerate() {
            let name_unique = format!("select result {} {}:{}", i, span.line, span.start);
            let selected_value =
                ConstrainedValue::conditionally_select(cs.ns(|| name_unique), &indicator, &result, &return_value)
                    .map_err(|_| {
                        StatementError::select_fail(result.to_string(), return_value.to_string(), span.clone())
                    })?;

            return_value = selected_value;
        }

        Ok(return_value)
    }
}
