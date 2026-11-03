// tidal-rules: CLS-26
// tidal-expect: reject
// tidal-expect-gcc-16: accept
// tidal-expect-gcc-host: accept
// tidal-error-clang: class member cannot be redeclared
// tidal-error-gcc: cannot be overloaded with
// tidal-error-edg: both have ref-qualifi
// tidal-note: Q-0027. GCC 16 accepts the declarations. Both overloads are const and take an int. One has the ref-qualifier &&, and the other has none.
struct S {
  int f(int) const &&;
  int f(int) const;
};
