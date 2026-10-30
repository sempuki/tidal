// tidal-rules: CTOR-13
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct S {
  int a;
  int b;
  S() : b(1), a(2) {}
};
