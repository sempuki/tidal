// tidal-rules: CLS-12
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
void f() {
  struct L { void g(); };
  void L::g() {}
}
