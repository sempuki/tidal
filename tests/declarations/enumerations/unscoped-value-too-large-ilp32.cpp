// tidal-rules: ENUM-5
// tidal-expect: reject
// tidal-flags: -m32
// tidal-error-clang: enumeration values exceed range of largest integer
// tidal-error-gcc: no integral type can represent all of the enumerator values
// tidal-error-edg: cannot be represented by the integral type underlying
// tidal-note: Q-0018. The values need a signed type wider than 64 bits. With -m32, GCC uses no 128-bit type for an enumeration, so every compiler rejects it.
enum Huge { A = 1ULL << 63, B = -1 };
