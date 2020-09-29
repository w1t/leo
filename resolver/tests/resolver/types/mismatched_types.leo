//
// Defines a variable `a` with explicit type `u8` and value `1u32`.
//
// Expected output: TypeError
// Message: "Expected type `u8`, found type `u32`"
//
function main() {
    let a: u8 = 1u32;
}