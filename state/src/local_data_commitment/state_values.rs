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

use crate::{find_input, input_to_bytes, input_to_integer_string, StateValuesError};
use leo_ast::State as AstState;

use std::convert::TryFrom;

static LEAF_INDEX_PARAMETER_STRING: &str = "leaf_index";
static ROOT_PARAMETER_STRING: &str = "root";

/// The serialized values included in the state.
/// A new [`StateValues`] type can be constructed from an [`AstState`] type.
pub struct StateValues {
    pub leaf_index: u32,
    pub root: Vec<u8>,
}

impl TryFrom<&AstState> for StateValues {
    type Error = StateValuesError;

    fn try_from(ast_state: &AstState) -> Result<Self, Self::Error> {
        let parameters = ast_state.values();

        // Lookup leaf index
        let leaf_index_value = find_input(LEAF_INDEX_PARAMETER_STRING.to_owned(), &parameters)?;
        let leaf_index = input_to_integer_string(leaf_index_value)?.parse::<u32>()?;

        // Lookup root
        let root_value = find_input(ROOT_PARAMETER_STRING.to_owned(), &parameters)?;
        let root = input_to_bytes(root_value)?;

        Ok(Self { leaf_index, root })
    }
}
