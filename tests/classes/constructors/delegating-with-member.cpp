// tidal-rules: CTOR-16
// tidal-expect: reject
// tidal-error-clang: delegating constructor must appear alone
// tidal-error-gcc: follows constructor delegation
// tidal-error-edg: delegating constructor cannot have other mem-initializers
struct S {
  int v;
  int w;
  S(int x) : v(x) {}
  S() : S(1), w(2) {}
};
