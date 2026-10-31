// tidal-rules: ENUM-5
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: enumeration values exceed range of largest integer
// tidal-error-edg: cannot be represented by the integral type underlying
// tidal-note: Q-0018. GCC uses a 128-bit underlying type; Clang and EDG reject.
enum Huge { A = 1ULL << 63, B = -1 };
