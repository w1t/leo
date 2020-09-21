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
use leo_typed::{Identifier, IntegerType, Type as UnresolvedType};
use serde::{Deserialize, Serialize};

use crate::SymbolTable;
use std::convert::TryFrom;

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
    /// Resolve a type inside of a circuit definition.
    /// If this type is SelfType, return the circuit's type
    pub fn from_unresolved_circuit_type(
        mut table: &mut SymbolTable,
        circuit_name: Identifier,
        type_: UnresolvedType,
    ) -> Self {
        match type_ {
            UnresolvedType::Address => Type::Address,
            UnresolvedType::Boolean => Type::Boolean,
            UnresolvedType::Field => Type::Field,
            UnresolvedType::Group => Type::Group,
            UnresolvedType::IntegerType(integer) => Type::IntegerType(integer),

            UnresolvedType::Array(type_, dimensions) => {
                let array_type = Type::from_unresolved_circuit_type(table, circuit_name, *type_);
                Type::Array(Box::new(array_type), dimensions)
            }
            UnresolvedType::Tuple(types) => {
                let tuple_types = types
                    .into_iter()
                    .map(|type_| Type::from_unresolved_circuit_type(table, circuit_name.clone(), type_))
                    .collect::<Vec<_>>();

                Type::Tuple(tuple_types)
            }

            UnresolvedType::Circuit(identifier) => {
                // Check that circuit exists
                let exists = table.get_variable(&identifier.name);
                if exists.is_none() {
                    unimplemented!("ERROR: undefined circuit type error")
                }

                Type::Circuit(identifier)
            }
            UnresolvedType::SelfType => Type::Circuit(circuit_name),
        }
    }
}
