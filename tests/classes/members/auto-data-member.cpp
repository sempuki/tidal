// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: auto. not allowed in non-static
// tidal-error-gcc: non-static data member declared with placeholder
// tidal-error-edg: auto. is not allowed here
struct S { auto x = 1; };
