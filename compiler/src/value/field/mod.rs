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

//! A field value in a compiled Leo program.

pub mod input;

pub mod field_type;
pub use self::field_type::*;
//
use serde::export::PhantomData;
use snark_std::{field::Field as FieldStd, traits::CircuitBuilder};
use snarkos_curves::bls12_377::Fr;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub struct FieldCircuitBuilder<F: Field + PrimeField, CS: ConstraintSystem<F>>(Rc<RefCell<CS>>, PhantomData<F>);

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> CircuitBuilder<F> for FieldCircuitBuilder<F, CS> {
    type CS = CS;

    fn borrow_mut(&self) -> RefMut<Self::CS> {
        self.0.borrow_mut()
    }
}

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> Clone for FieldCircuitBuilder<F, CS> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

// impl<F: Field + PrimeField, CS: ConstraintSystem<F>> FieldCircuitBuilder<F, CS> {
//     pub fn new(cs: CS) -> Self {
//         Self { 0: Rc::new(RefCell::new(())), 1: Default::default() }
//     }
//
// }
