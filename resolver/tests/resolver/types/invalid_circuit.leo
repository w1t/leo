//
// Defines a circuit `Foo { x: u32 }`.
// Defines a function `foo() {}`.
// Attempts to access variable `foo.x`.
//
// Expected output: TypeError
// Message: "Expected circuit type, found type `function foo`."
//

circuit Foo { x: u32 }

function foo() {}

function main() {
    let a = foo.x;
}