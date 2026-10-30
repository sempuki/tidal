// tidal-rules: CTOR-12
// tidal-expect: reject
struct S {
  int a;
  int b;
  constexpr S() : a(b), b(1) {}
};
constexpr S s;
