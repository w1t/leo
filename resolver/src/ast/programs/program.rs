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

use crate::{ResolvedNode, SymbolTable};
use leo_typed::programs::Program as TypedProgram;

pub static MAIN_FUNCTION_NAME: &str = "main";

/// A Leo program with resolved types and semantic analysis
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    // pub imports: Vec<Import>,
// pub circuits: HashMap<Identifier, Circuit>,
// pub function: HashMap<Identifier, Function>,
}

impl ResolvedNode for Program {
    type Error = ();
    type UnresolvedNode = TypedProgram;

    /// Returns a resolved program AST given an unresolved program AST
    fn resolve(_table: &mut SymbolTable, _unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        // let mut circuits = HashMap::new();

        // Resolve import statements

        // Resolve circuit definitions
        // unresolved.circuits.into_iter().for_each(|(identifier, circuit)| {
        //     let resolved_circuit = Circuit::resolve(&mut table, circuit).unwrap();
        //
        //     circuits.insert(identifier, resolved_circuit);
        // });

        // Resolve function definitions

        // Resolve tests

        // Look for main function
        // let main = unresolved.functions.into_iter().find(|(identifier, _)| {
        //     identifier.name.eq(MAIN_FUNCTION_NAME)
        // });
        //
        // //TODO: return no main function error
        // let program = match main {
        //     Some((_identifier, function)) => ,
        //     None => unimplemented!("ERROR: main function not found"),
        // }

        Ok(Program {})
    }
}
