// tidal-rules: DEP-17
// tidal-expect: reject
// tidal-error-gcc: has no member named .missing.
// tidal-error-clang: no member named .missing. in
// tidal-error-edg: has no member .missing.
// tidal-note: The class template has no dependent base, and no specialization declares missing. The instantiation of read fails on every compiler.
template <class T> struct Gauge {
  int read() { return this->missing(); }
};
int x = Gauge<int>{}.read();
