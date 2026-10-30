// tidal-rules: CTOR-8
// tidal-expect: reject
struct S {
  S() : S() {}
};
