// tidal-rules: CTOR-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0053. Clang and EDG reject an indirect delegation cycle. GCC accepts it.
struct S {
  S(int) : S('a') {}
  S(char) : S(1) {}
};
