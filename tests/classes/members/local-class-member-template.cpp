// tidal-rules: CLS-12
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
void f() {
  struct L { template <class T> void g(T) {} };
}
