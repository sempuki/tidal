// tidal-rules: BF-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0058. GCC accepts a cv-qualifier on an unnamed bit-field. Clang and EDG reject it.
struct S { const int : 3; };
