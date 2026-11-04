// tidal-rules: LIST-2 CONV-21
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: invalid narrowing conversion
// tidal-note: A conversion from a floating-point type to an integral type narrows, and bool is an integral type. The constant 1.0 does not help.
bool b{1.0};
bool c{2.5};
