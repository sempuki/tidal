// tidal-rules: ASN-12
// tidal-expect: reject
// tidal-error-clang: assigning to .int. from incompatible type .S.
// tidal-error-gcc: cannot convert .S. to .int. in assignment
// tidal-error-edg: no suitable conversion function from .S. to .int.
// tidal-note: The right operand is a class with no conversion to int.
struct S {};
void f(int& i) { i = S{}; }
