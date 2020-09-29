//
// Defines a variable `a` with explicit type `Foo` and value `1u8`.
//
// Expected output: TypeError
// Message: "Type circuit `Foo` must be defined before it is used in an expression."
//
function main() {
    let a: Foo = 1u8;
}