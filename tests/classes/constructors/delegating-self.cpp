// tidal-rules: CTOR-8
// tidal-expect: reject
// tidal-error-clang: creates a delegation cycle
// tidal-error-gcc: constructor delegates to itself
// tidal-error-edg: delegates directly or indirectly to itself
struct S {
  S() : S() {}
};
