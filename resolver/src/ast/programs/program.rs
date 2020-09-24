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

use crate::{Circuit, Function, ResolvedNode, SymbolTable, TestFunction};
use leo_typed::{programs::Program as TypedProgram, Identifier};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static MAIN_FUNCTION_NAME: &str = "main";

/// A Leo program with resolved types and semantic analysis
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    // pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
}

impl ResolvedNode for Program {
    type Error = ();
    type UnresolvedNode = TypedProgram;

    /// Returns a resolved program AST given an unresolved program AST
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();

        // Resolve import statements

        // Resolve circuit definitions
        unresolved.circuits.into_iter().for_each(|(identifier, circuit)| {
            let resolved_circuit = Circuit::resolve(table, circuit).unwrap();

            circuits.insert(identifier, resolved_circuit);
        });

        // Resolve function statements
        unresolved.functions.into_iter().for_each(|(identifier, function)| {
            let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));
            let resolved_function = Function::resolve(&mut child_table, function).unwrap();

            functions.insert(identifier, resolved_function);
        });

        // Resolve tests
        unresolved.tests.into_iter().for_each(|(identifier, test)| {
            let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));
            let resolved_test = TestFunction::resolve(&mut child_table, test).unwrap();

            tests.insert(identifier, resolved_test);
        });

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

        Ok(Program { circuits, functions })
    }
}
