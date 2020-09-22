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

use crate::{CircuitType, Function, ResolvedNode, SymbolTable};
use leo_typed::{circuit::Circuit as UnresolvedCircuit, identifier::Identifier};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A resolved circuit definition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Circuit {
    /// The name of the circuit definition
    pub type_: CircuitType,
    /// The circuit member functions
    pub functions: HashMap<Identifier, Function>,
}

impl ResolvedNode for Circuit {
    type Error = ();
    type UnresolvedNode = UnresolvedCircuit;

    /// Returns a resolved circuit AST given an unresolved circuit AST
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        Err(())
    }
}
