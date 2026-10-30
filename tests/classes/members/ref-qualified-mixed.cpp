// tidal-rules: CLS-6
// tidal-expect: reject
// tidal-expect-gcc-16: accept
// tidal-expect-gcc-host: accept
// tidal-note: Q-0027. GCC 16 accepts the declarations and reports ambiguity only at a call.
struct S {
  void f() &;
  void f();
};
