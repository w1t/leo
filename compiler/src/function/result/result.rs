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
        results: Vec<(Option<Boolean>, ConstrainedValue<F, G>)>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Initialize empty return value
        let mut return_value = ConstrainedValue::Tuple(vec![]);

        // if the function does not expect a return type, make sure there are no returned results.
        let return_type = match expected_return {
            Some(return_type) => return_type,
            None => {
                if results.len() == 0 {
                    return Ok(return_value);
                } else {
                    return Err(StatementError::invalid_number_of_returns(
                        0,
                        results.len(),
                        span.clone(),
                    ));
                }
            }
        };

        // If all indicators are none, then there are no branch conditions in the function.
        // We simply return the last result.

        if let None = results.iter().find(|(indicator, _res)| indicator.is_some()) {
            let result = &results[results.len() - 1].1;

            return Ok(result.clone());
        }

        // Find the return value
        let mut ignored = vec![];
        let found_return = false;
        for (indicator, result) in results.into_iter() {
            if let Some(indicator_bool) = indicator {
                if indicator_bool {
                    // Error if we already have a return value
                    if found_return {
                        return Err();
                    } else {
                    }
                }
            }
        }

        // If there are branches in the function we need to use the `ConditionalSelectGadget` to parse through and select the correct one.
        for (i, (indicator, result)) in results.into_iter().enumerate() {
            // Error if a statement returned a result with an incorrect type
            let result_type = result.to_type(span.clone())?;
            if return_type != result_type {
                return Err(StatementError::arguments_type(&return_type, &result_type, span.clone()));
            }

            let condition = indicator.unwrap_or(Boolean::Constant(true));
            let name_unique = format!("select {} {}:{}", result, span.line, span.start);
            let selected_value =
                ConstrainedValue::conditionally_select(cs.ns(|| name_unique), &condition, &result, return_value)
                    .map_err(|_| {
                        StatementError::select_fail(result.to_string(), return_value.to_string(), span.clone())
                    })?;

            *return_value = selected_value;
        }

        Ok(())
    }
}
