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

use crate::TestLeoResolvedAst;

///
/// Defines a variable `a` with explicit type `[u8; 32]` and value `1u8`.
///
/// Expected output: TypeError
/// Message: "Expected type `[u8; (32)]`, found type `u8`."
///
#[test]
fn test_invalid_array() {
    let program_bytes = include_bytes!("invalid_array.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Defines a circuit `Foo { x: u32 }`.
/// Defines a function `foo() {}`.
/// Attempts to access variable `foo.x`.
///
/// Expected output: TypeError
/// Message: "Expected circuit type, found type `function foo`."
///
#[test]
fn test_invalid_circuit() {
    let program_bytes = include_bytes!("invalid_circuit.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Defines a circuit `Foo {}`.
/// Attempts to call the circuit like a function using parenthesis `Foo()`.
///
/// Expected output: TypeError
/// Message: "Expected function type, found type `circuit Foo`."
///
#[test]
fn test_invalid_function() {
    let program_bytes = include_bytes!("invalid_function.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Attempts to compare boolean values `false < true`.
///
/// Expected output: TypeError
/// Message: "Expected integer type, found type `bool`."
///
#[test]
fn test_invalid_integer() {
    let program_bytes = include_bytes!("invalid_integer.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Defines a variable `a` with value `1u8`.
/// Attempts to access the first tuple index `a.0`.
///
/// Expected output: TypeError
/// Message: "Expected tuple type, found type `u8`."
///
#[test]
fn test_invalid_tuple() {
    let program_bytes = include_bytes!("invalid_tuple.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Defines a variable `a` with explicit type `u8` and value `1u32`.
///
/// Expected output: TypeError
/// Message: "Expected type `u8`, found type `u32`."
///
#[test]
fn test_mismatched_types() {
    let program_bytes = include_bytes!("mismatched_types.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Defines a variable `a` with explicit type `Self` and value `1u32`.
///
/// Expected output: TypeError
/// Message: "Type `Self` is only available in circuit definitions and circuit functions."
///
#[test]
fn test_self_not_available() {
    let program_bytes = include_bytes!("self_not_available.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Defines a variable `a` with explicit type `Foo` and value `1u8`.
///
/// Expected output: TypeError
/// Message: "Type circuit `Foo` must be defined before it is used in an expression."
///
#[test]
fn test_undefined_circuit() {
    let program_bytes = include_bytes!("undefined_circuit.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}

///
/// Defines circuit `Foo { a: u32 }`.
/// Attempts to initialize circuit `Foo { b: 1u32 }`.
///
/// Expected output: TypeError
/// Message: "Circuit has no member variable named `b`."
///
#[test]
fn test_undefined_circuit_member() {
    let program_bytes = include_bytes!("undefined_circuit_member.leo");
    let resolver = TestLeoResolvedAst::new(program_bytes);

    resolver.expect_resolver_error();
}
