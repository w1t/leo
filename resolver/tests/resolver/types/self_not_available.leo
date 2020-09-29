//
// Defines a variable `a` with explicit type `Self` and value `1u32`.
//
// Expected output: TypeError
// Message: "Type `Self` is only available in circuit definitions and circuit functions"
//

function main() {
    let a: Self = 1u32;
}