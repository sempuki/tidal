// tidal-rules: TR-10
// tidal-expect: reject
// tidal-error-clang: outside the range of representable values
// tidal-error-gcc: overflow in constant expression
// tidal-error-edg: value exceeds range
// tidal-note: Signed overflow in an addition makes the evaluation not constant.
constexpr int next(int x) { return x + 1; }
static_assert(next(2147483647) < 0);
