// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: not allowed in block scope
// tidal-error-gcc: specifier invalid for function .* declared out of global scope
// tidal-error-edg: .inline. is not allowed
void f() { inline void g(); }
