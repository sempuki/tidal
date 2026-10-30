// tidal-rules: CTOR-16
// tidal-expect: reject
struct S {
  int v;
  int w;
  S(int x) : v(x) {}
  S() : S(1), w(2) {}
};
