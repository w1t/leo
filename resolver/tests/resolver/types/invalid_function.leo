//
// Defines a circuit `Foo {}`.
// Attempts to call the circuit like a function using parenthesis `Foo()`.
//
// Expected output: TypeError
// Message: "Expected function type, found type `circuit Foo`"
//
circuit Foo {}

function main() {
    let a = Foo();
}