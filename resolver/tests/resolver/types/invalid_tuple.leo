//
// Defines a variable `a` with value `1u8`.
// Attempts to access the first tuple index `a.0`.
//
// Expected output: TypeError
// Message: "Expected tuple type, found type `u8`"
//
function main() {
    let a = 1u8;
    let b = a.0;
}