// tidal-rules: UNI-8
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed to type .char
// tidal-error-gcc: narrowing conversion of .1000
// tidal-error-edg: invalid narrowing conversion
union U { char c; int i; };
U u = {1000};
