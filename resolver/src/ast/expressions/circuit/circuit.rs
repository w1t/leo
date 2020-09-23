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
use crate::{CircuitVariableDefinition, Expression, ExpressionValue, ResolvedNode, SymbolTable, Type};
use leo_typed::{CircuitVariableDefinition as UnresolvedCircuitVariableDefinition, Identifier, Span};

impl Expression {
    /// Resolves an inline circuit expression
    pub(crate) fn circuit(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        identifier: Identifier,
        variables: Vec<UnresolvedCircuitVariableDefinition>,
        span: Span,
    ) -> Result<Self, ()> {
        // Check expected type
        let type_ = Type::Circuit(identifier.clone());
        Type::check_type(&expected_type, &type_, span.clone()).unwrap();

        // Lookup circuit in symbol table
        let circuit_option = table.get_circuit(&identifier);

        //TODO: throw error for undefined circuit
        let circuit = match circuit_option {
            Some(circuit) => circuit,
            None => unimplemented!("ERROR: circuit not found"),
        };

        // Check the number of variables given
        let expected_variables = circuit.variables.clone();

        if variables.len() != expected_variables.len() {
            unimplemented!("ERROR: circuit member lengths not equal")
        }

        // Check the name and type for each circuit variable
        let mut variables_resolved = vec![];

        for variable in variables.into_iter() {
            // Find variable by name
            let matched_variable = expected_variables
                .iter()
                .find(|variable| variable.identifier.eq(&variable.identifier));

            let variable_type = match matched_variable {
                Some(variable_type) => variable_type,
                None => unimplemented!("ERROR: Unknown circuit member"),
            };

            // Resolve the variable expression using the expected variable type
            let expected_variable_type = Some(variable_type.type_.clone());

            let variable_resolved =
                CircuitVariableDefinition::resolve(table, (expected_variable_type, variable)).unwrap();

            variables_resolved.push(variable_resolved);
        }

        Ok(Expression {
            type_,
            value: ExpressionValue::Circuit(identifier, variables_resolved, span),
        })
    }
}
