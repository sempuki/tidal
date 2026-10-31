// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: must be .bool., not .auto
// tidal-error-gcc: must return .bool
// tidal-error-edg: must be .bool
struct S { int x; friend auto operator==(const S&, const S&) = default; };
bool b = S{1} == S{1};
