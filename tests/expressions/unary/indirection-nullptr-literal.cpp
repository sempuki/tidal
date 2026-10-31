// tidal-rules: UN-6
// tidal-expect: reject
// tidal-error-clang: indirection requires pointer operand
// tidal-error-gcc: invalid type argument of unary .\*.
// tidal-error-edg: must be a pointer
auto x = *nullptr;
