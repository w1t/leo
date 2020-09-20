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
use crate::ast::Attribute;
use leo_typed::{Circuit, Identifier, Type};

use std::convert::TryFrom;

/// A symbol table entry stores the type and attribute information for an identifier
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Entry {
    identifier: Identifier,
    type_: Type,
    attributes: Vec<Attribute>,
}

impl TryFrom<Circuit> for Entry {
    type Error = ();

    fn try_from(value: Circuit) -> Result<Self, Self::Error> {
        let identifier = value.circuit_name;
        let type_ = Type::from(identifier.clone());
        let attributes = vec![];

        Ok(Entry {
            identifier,
            type_,
            attributes,
        })
    }
}
