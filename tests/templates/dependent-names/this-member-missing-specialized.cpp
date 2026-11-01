// tidal-rules: DEP-30 DEP-17
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-expect-clang-18: accept
// tidal-error-gcc: has no member named .missing.
// tidal-error-clang: no member named .missing. in
// tidal-error-edg: has no member .missing.
// tidal-note: Q-0228. The template is never instantiated. GCC 14 and Clang 18 accept it, and DEP-30 allows both results. An explicit specialization of the class does not change the lookup in the primary template, which has no dependent base.
template <class T> struct Gauge {
  int read() { return this->missing; }
};
template <> struct Gauge<int> { int missing = 1; };
