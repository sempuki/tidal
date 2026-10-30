// tidal-rules: ENUM-5
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0018. GCC uses a 128-bit underlying type; Clang and EDG reject.
enum Huge { A = 1ULL << 63, B = -1 };
