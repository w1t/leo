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

use crate::{FunctionType, ResolvedNode, Statement, SymbolTable};
use leo_typed::Function as UnresolvedFunction;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    /// The name of the function definition
    pub type_: FunctionType,
    /// The function statements
    pub statements: Vec<Statement>,
}

impl ResolvedNode for Function {
    type Error = ();
    type UnresolvedNode = UnresolvedFunction;

    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let identifier = unresolved.identifier;
        // TODO: Throw an unknown function error
        let type_ = table.get_function(&identifier).unwrap();

        // Create function context
        let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));

        // Insert function inputs into symbol table
        for input in type_.inputs.clone() {
            // TODO: throw duplicate function input error
            input.insert(&mut child_table).is_some();
        }

        // Pass expected function output to resolved statements
        let output = type_.output.clone();
        let mut statements = vec![];

        // Resolve all function statements
        for (_i, unresolved_statement) in unresolved.statements.into_iter().enumerate() {
            let statement = Statement::resolve(&mut child_table, (output.clone(), unresolved_statement)).unwrap();

            statements.push(statement);
        }

        Ok(Function {
            type_: type_.clone(),
            statements,
        })
    }
}
