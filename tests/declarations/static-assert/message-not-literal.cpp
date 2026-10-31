// tidal-rules: SA-8
// tidal-expect: reject
// tidal-error-clang: string literal
// tidal-error-gcc: non-string message
// tidal-error-edg: expected a string literal
constexpr const char* msg = "x";
static_assert(true, msg);
