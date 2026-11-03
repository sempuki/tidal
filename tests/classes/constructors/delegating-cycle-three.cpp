// tidal-rules: CTOR-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: creates a delegation cycle
// tidal-error-edg: delegates directly or indirectly to itself
// tidal-note: Q-0053. A cycle through three constructors. Clang and EDG reject an indirect delegation cycle. GCC accepts it.
struct S {
  S(int) : S(1.0) {}
  S(double) : S('a') {}
  S(char) : S(1) {}
};
