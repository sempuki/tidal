// tidal-rules: ENUM-14
// tidal-expect: reject
// tidal-error-clang: not representable in the underlying type
// tidal-error-gcc: outside the range of underlying type
// tidal-error-edg: outside the range of its underlying type
enum class E { A = 2147483647, B };
