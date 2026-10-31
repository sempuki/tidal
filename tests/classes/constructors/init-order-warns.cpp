// tidal-rules: CTOR-13
// tidal-expect: reject
// tidal-error-clang: will be initialized after
// tidal-error-gcc: will be initialized after
// tidal-flags: -Wall -Werror
struct S {
  int a;
  int b;
  S() : b(1), a(2) {}
};
