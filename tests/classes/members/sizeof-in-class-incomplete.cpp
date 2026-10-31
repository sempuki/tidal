// tidal-rules: CLS-2
// tidal-expect: reject
// tidal-error-clang: sizeof. to an incomplete type
// tidal-error-gcc: sizeof. to incomplete type
// tidal-error-edg: incomplete type .S. is not allowed
struct S {
  int a;
  static constexpr int size = sizeof(S);
};
