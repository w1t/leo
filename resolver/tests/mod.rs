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

pub mod symbol_table;

use leo_ast::LeoAst;
use leo_resolver::{LeoResolvedAst, ResolverError};
use leo_typed::LeoTypedAst;
use std::path::PathBuf;

const TEST_PROGRAM_PATH: &str = "";

/// A helper struct to test a `LeoResolvedAst`
pub struct TestLeoResolvedAst {
    file_path: PathBuf,
    typed: LeoTypedAst,
}

impl TestLeoResolvedAst {
    /// Returns a typed AST given a leo program
    pub fn new(bytes: &[u8]) -> Self {
        let file_string = String::from_utf8_lossy(bytes);
        let file_path = PathBuf::from(TEST_PROGRAM_PATH);

        // 1. Get parser AST
        let ast = LeoAst::new(&file_path, &*file_string).unwrap();

        // 2. Get typed AST
        let typed = LeoTypedAst::new(TEST_PROGRAM_PATH, &ast);

        Self { file_path, typed }
    }

    /// Parse the typed AST into a `LeoResolvedAst`. Expect no errors during parsing.
    pub fn expect_success(self) {
        // 3. Get resolved AST
        let resolved = LeoResolvedAst::new(self.typed, self.file_path).is_ok();

        assert!(resolved)
    }

    /// Parse the typed AST into a `LeoResolvedAst`. Expect an error involving identifiers in the symbol table.
    pub fn expect_symbol_table_error(self) {
        // 3. Get resolved AST
        let resolved = LeoResolvedAst::new(self.typed, self.file_path).unwrap_err();

        match resolved {
            ResolverError::SymbolTableError(_) => {} // Ok
            error => panic!("Expected a symbol table error found `{}`", error),
        }
    }
}

// Tests that a `TestLeoResolvedAst` struct can be created from a Leo program
#[test]
fn test_resolver_new() {
    let program_bytes = include_bytes!("empty.leo");
    TestLeoResolvedAst::new(program_bytes);
}
