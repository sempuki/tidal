// tidal-rules: INIT-7
// tidal-expect: reject
// tidal-error-clang: cannot initialize non-class type .int. with a parenthesized initializer list
// tidal-error-gcc: list-initializer for non-class type must not be parenthesized
// tidal-error-edg: parenthesized initializer for a non-class entity must be an expression
int x({1});
