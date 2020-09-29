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

use crate::{Program, ResolverError, SymbolTable};
use leo_imports::ImportParser;
use leo_typed::LeoTypedAst;
use std::path::PathBuf;

/// A resolved syntax tree is represented as a `Program` without implicitly typed values.
#[derive(Debug, Eq, PartialEq)]
pub struct LeoResolvedAst {
    pub resolved_ast: Program,
}

impl LeoResolvedAst {
    /// Creates a new resolved syntax tree from a given typed syntax tree
    pub fn new(ast: LeoTypedAst, path: PathBuf) -> Result<Self, ResolverError> {
        // Get AST's for main program + imported programs
        let program = ast.into_repr();
        let _imported_programs = ImportParser::parse(&program)?;

        //todo: load main function `input` register and state file types

        // Create a symbol table for main.leo
        let mut symbol_table = SymbolTable::new(None);

        // Pass 1: Insert circuits and functions as variable types
        symbol_table.insert_program_variables(&program).map_err(|mut e| {
            e.set_path(path);

            e
        })?;

        // Pass 2: Insert circuits and functions as definitions
        symbol_table.insert_definitions(&program)?;

        // Pass 2: Perform semantic analysis on program
        // At each AST node:
        //    1. Resolve all child AST nodes
        //    2. Resolve current AST node
        let resolved_ast = Program::resolve(&mut symbol_table, program)?;

        Ok(LeoResolvedAst { resolved_ast })
    }
}

///
/// A node in the `LeoResolvedAST`. This node and all of its children should not contain any implicit types
///
pub trait ResolvedNode {
    /// The expected error type if the type resolution fails
    type Error;

    /// The `leo-typed` AST node that we are type checking
    type UnresolvedNode;

    /// Returns a resolved AST representation given an unresolved AST representation and symbol table
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized;
}
