// tidal-rules: TPL-33
// tidal-expect: reject
// tidal-error-clang: cannot be defined in a type specifier
// tidal-error-gcc: types may not be defined in parameter types
// tidal-error-edg: type definition is not allowed
template <enum Color { red, green } C> struct Paint {};
