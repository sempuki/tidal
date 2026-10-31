// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: invalid parameter type for defaulted
// tidal-error-gcc: must have parameter type
// tidal-error-edg: bad type .* for parameter of defaulted comparison
struct T {};
struct S { int x; bool operator==(const T&) const = default; };
