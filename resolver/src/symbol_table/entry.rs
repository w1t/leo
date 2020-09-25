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
use crate::ast::{Attribute, Type};
use leo_typed::{Circuit, Function, Identifier};

use crate::FunctionInputVariableType;
use std::fmt;

/// A symbol table entry stores the type and attribute information for an identifier
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Entry {
    pub identifier: Identifier,
    pub type_: Type,
    pub attributes: Vec<Attribute>,
}

impl Entry {
    /// Returns the type of the variable
    pub fn type_(&self) -> &Type {
        &self.type_
    }

    /// Returns `true` if this variable's value can be modified
    pub fn is_mutable(&self) -> bool {
        self.attributes.contains(&Attribute::Mutable)
    }

    /// Returns `true` if this variable's value is static
    pub fn is_static(&self) -> bool {
        self.attributes.contains(&Attribute::Static)
    }
}

impl From<Circuit> for Entry {
    fn from(value: Circuit) -> Self {
        let identifier = value.circuit_name;

        Entry {
            identifier: identifier.clone(),
            type_: Type::Circuit(identifier),
            attributes: vec![],
        }
    }
}

impl From<Function> for Entry {
    fn from(value: Function) -> Self {
        let identifier = value.identifier;

        Entry {
            identifier: identifier.clone(),
            type_: Type::Function(identifier.clone()),
            attributes: vec![],
        }
    }
}

impl From<FunctionInputVariableType> for Entry {
    fn from(value: FunctionInputVariableType) -> Self {
        Entry {
            identifier: value.identifier,
            type_: value.type_,
            attributes: value.attributes,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
