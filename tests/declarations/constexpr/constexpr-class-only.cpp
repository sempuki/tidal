// tidal-rules: CX-5
// tidal-expect: reject
// tidal-error-clang: struct cannot be marked constexpr
// tidal-error-gcc: constexpr. cannot be used for type declarations
// tidal-error-edg: constexpr. is not valid here
constexpr struct S { int v; };
