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
use crate::SymbolTable;

use leo_typed::{Identifier, IntegerType, Span, Type as UnresolvedType};
use serde::{Deserialize, Serialize};

/// The type of an identifier in a Leo program. Cannot be implicit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    IntegerType(IntegerType),

    // Data type wrappers
    Array(Box<Type>, Vec<usize>),
    Tuple(Vec<Type>),

    // User defined types
    Circuit(Identifier),
    Function(Identifier),
}

impl Type {
    /// Resolves the given type. Cannot be an implicit or Self type.
    pub fn from_unresolved(table: &SymbolTable, type_: UnresolvedType) -> Self {
        match type_ {
            UnresolvedType::Address => Type::Address,
            UnresolvedType::Boolean => Type::Boolean,
            UnresolvedType::Field => Type::Field,
            UnresolvedType::Group => Type::Group,
            UnresolvedType::IntegerType(integer) => Type::IntegerType(integer),

            UnresolvedType::Array(type_, dimensions) => {
                let array_type = Type::from_unresolved(table, *type_);
                Type::Array(Box::new(array_type), dimensions)
            }
            UnresolvedType::Tuple(types) => {
                let tuple_types = types
                    .into_iter()
                    .map(|type_| Type::from_unresolved(table, type_))
                    .collect::<Vec<_>>();

                Type::Tuple(tuple_types)
            }

            UnresolvedType::Circuit(identifier) => {
                // Check that circuit exists
                let exists = table.get_variable(&identifier.name);
                // TODO: throw error for undefined circuit type
                if exists.is_none() {
                    unimplemented!("ERROR: undefined circuit type error")
                }

                Type::Circuit(identifier)
            }
            // TODO: throw error for invalid self type use
            UnresolvedType::SelfType => unimplemented!("ERROR: SelfType does not refer to a valid circuit definition"),
        }
    }

    /// Resolve a type inside of a circuit definition.
    /// If this type is SelfType, return the circuit's type
    pub fn from_circuit(table: &SymbolTable, circuit_name: Identifier, type_: UnresolvedType) -> Self {
        match type_ {
            UnresolvedType::Array(type_, dimensions) => {
                let array_type = Type::from_circuit(table, circuit_name, *type_);
                Type::Array(Box::new(array_type), dimensions)
            }
            UnresolvedType::Tuple(types) => {
                let tuple_types = types
                    .into_iter()
                    .map(|type_| Type::from_circuit(table, circuit_name.clone(), type_))
                    .collect::<Vec<_>>();

                Type::Tuple(tuple_types)
            }
            UnresolvedType::SelfType => Type::Circuit(circuit_name),
            // The unresolved type does not depend on the current circuit definition
            unresolved => Type::from_unresolved(table, unresolved),
        }
    }

    /// Returns `Ok` if the the expected type is `Some` && expected type == actual type
    pub fn check_type(expected_option: &Option<Self>, actual: &Type, _span: Span) -> Result<(), ()> {
        if let Some(expected) = expected_option {
            if expected.ne(&actual) {
                // TODO: throw mismatched type error
                unimplemented!("ERROR: mismatched types")
            }
        }
        Ok(())
    }

    /// Returns `Ok` if the expected type is `Some(IntegerType)`
    pub fn check_type_integer(expected: &Self, _span: Span) -> Result<(), ()> {
        match expected {
            Type::IntegerType(_) => Ok(()),
            // TODO: throw mismatched type error
            _ => unimplemented!("ERROR: mismatched types, expected integer"),
        }
    }
}
