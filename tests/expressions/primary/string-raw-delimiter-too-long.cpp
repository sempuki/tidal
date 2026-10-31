// tidal-rules: PRIM-19
// tidal-expect: reject
// tidal-error-clang: raw string delimiter longer than 16 characters
// tidal-error-gcc: raw string delimiter longer than 16 characters
// tidal-error-edg: not found within 16 characters
auto s = R"0123456789abcdefg(x)0123456789abcdefg";
