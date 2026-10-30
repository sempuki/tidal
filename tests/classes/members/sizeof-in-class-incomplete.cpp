// tidal-rules: CLS-2
// tidal-expect: reject
struct S {
  int a;
  static constexpr int size = sizeof(S);
};
