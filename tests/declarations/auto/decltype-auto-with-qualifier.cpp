// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: cannot be combined with other type specifiers
// tidal-error-gcc: cannot be cv-qualified
// tidal-error-edg: cannot have added type qualifiers
const decltype(auto) a = 1;
