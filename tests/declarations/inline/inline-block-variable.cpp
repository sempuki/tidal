// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: not allowed in block scope
// tidal-error-gcc: specifier invalid for variable .* declared at block scope
// tidal-error-edg: .inline. is not allowed
void f() { inline int x = 0; }
