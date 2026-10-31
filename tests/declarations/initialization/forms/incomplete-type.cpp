// tidal-rules: INIT-7
// tidal-expect: reject
// tidal-error-clang: incomplete type .I
// tidal-error-gcc: has initializer but incomplete type
// tidal-error-edg: incomplete type .I. is not allowed
struct I;
I j{};
