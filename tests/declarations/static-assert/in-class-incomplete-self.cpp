// tidal-rules: SA-6
// tidal-expect: reject
// tidal-error-clang: sizeof.*incomplete type
// tidal-error-gcc: sizeof.*incomplete type
// tidal-error-edg: incomplete type
struct S { int a; static_assert(sizeof(S) > 0); };
