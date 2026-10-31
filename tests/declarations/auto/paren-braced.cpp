// tidal-rules: AUTO-20
// tidal-expect: reject
// tidal-error-clang: from parenthesized initializer list
// tidal-error-gcc: list-initializer for non-class type must not be parenthesized
// tidal-error-edg: parenthesized initializer must be an expression
auto a({1});
