// tidal-rules: CTOR-12
// tidal-expect: reject
// tidal-error-gcc: accessing uninitialized member .S::b
struct S {
  int a;
  int b;
  constexpr S() : a(b), b(1) {}
};
constexpr S s;
