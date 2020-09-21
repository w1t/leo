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

use crate::{Attribute, Type};
use leo_typed::{Identifier, Type as UnresolvedType};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircuitVariableType {
    /// The name of the circuit member variable
    pub identifier: Identifier,
    /// The type of the circuit member variable
    pub type_: Type,
    /// The attributes of the circuit member variable
    pub attributes: Vec<Attribute>,
}

// impl CircuitVariableType {
//     pub fn from_unresolved(
//         circuit_name: Identifier,
//         mutable: bool,
//         identifier: Identifier,
//         unresolved_type: UnresolvedType,
//     ) -> Self {
//         let resolved_type = Type::from_unresolved_circuit_type(circuit_name, unresolved_type).unwrap();
//     }
// }
