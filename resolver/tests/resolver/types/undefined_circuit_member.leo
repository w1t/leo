//
// Defines circuit `Foo { a: u32 }`.
// Attempts to initialize circuit `Foo { b: 1u32 }`.
//
// Expected output: TypeError
// Message: "Circuit has no member variable named `b`."
//
circuit Foo { a: u32 }

function main() {
    let f: Foo = Foo { b: 1u32 };
}