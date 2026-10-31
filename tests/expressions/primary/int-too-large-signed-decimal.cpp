// tidal-rules: PRIM-2
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: too large to be represented in a signed integer type
// tidal-error-gcc: so large that it is unsigned
// tidal-note: Q-0064. Clang and GCC reject under -pedantic-errors and warn without it. EDG 7.0 accepts it.
auto x = 9223372036854775808;
