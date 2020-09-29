//
// Defines a variable `a` with explicit type `[u8; 32]` and value `1u8`.
//
// Expected output: TypeError
// Message: "Expected type `[u8; (32)]`, found type `u8`."
//
function main() {
    let a: [u8; 32] = 1u8;
}