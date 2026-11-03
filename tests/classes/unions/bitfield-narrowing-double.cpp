// tidal-rules: BF-11
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
// tidal-note: The braced initializer is a double for an int bit-field.
struct S { int x : 4; };
S s{1.5};
