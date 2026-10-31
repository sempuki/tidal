// tidal-rules: PRIM-19
// tidal-expect: reject
// tidal-error-clang: in raw string delimiter
// tidal-error-gcc: in raw string delimiter
// tidal-error-edg: invalid character in raw string delimiter
auto s = R"a\b(x)a\b";
