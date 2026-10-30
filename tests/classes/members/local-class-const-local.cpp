// tidal-rules: CLS-12
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
int f() {
  const int limit = 5;
  struct L { int get() { return limit; } };
  return L{}.get();
}
