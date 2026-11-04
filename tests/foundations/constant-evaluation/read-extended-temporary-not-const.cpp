// tidal-rules: CE-3
// tidal-expect: reject
// tidal-error-clang: read of temporary
// tidal-error-gcc: temporary.* is not usable in a constant expression
// tidal-error-edg: run-time storage
constexpr int&& count = 4;
static_assert(count == 4);
