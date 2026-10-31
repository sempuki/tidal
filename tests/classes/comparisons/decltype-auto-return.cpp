// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: must be .bool
// tidal-error-gcc: must return .bool
// tidal-error-edg: must be .bool
struct S { int x; decltype(auto) operator==(const S&) const = default; };
