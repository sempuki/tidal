// tidal-rules: CTOR-13
// tidal-expect: accept
// tidal-flags: -Werror
// tidal-note: Clang and GCC give the order warning only under -Wall, so -Werror alone accepts the program.
struct S {
  int a;
  int b;
  S() : b(1), a(2) {}
};
