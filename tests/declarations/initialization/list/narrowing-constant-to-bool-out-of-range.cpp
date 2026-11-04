// tidal-rules: LIST-2 CONV-21
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: invalid narrowing conversion
// tidal-note: bool holds only 0 and 1, so the constant 2 does not fit.
bool b{2};
