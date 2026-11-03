// tidal-rules: CTOR-8
// tidal-expect: reject
// tidal-error-clang: creates a delegation cycle
// tidal-error-gcc: constructor delegates to itself
// tidal-error-edg: delegates directly or indirectly to itself
// tidal-note: The constructor passes its own parameter to itself, so overload resolution chooses the same constructor.
struct S {
  int v;
  S(int x) : S(x) {}
};
