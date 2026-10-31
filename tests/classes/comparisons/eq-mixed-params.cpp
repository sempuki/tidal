// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: must have the same type
// tidal-error-gcc: must have parameters of either type
// tidal-error-edg: bad type .* for parameter of defaulted comparison
struct S { int x; friend bool operator==(const S&, S) = default; };
