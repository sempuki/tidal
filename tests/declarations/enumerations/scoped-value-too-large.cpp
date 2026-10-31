// tidal-rules: ENUM-17
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed to type .int
// tidal-error-gcc: outside the range of underlying type
// tidal-error-edg: outside the range of its underlying type
enum class E { A = 1LL << 40 };
