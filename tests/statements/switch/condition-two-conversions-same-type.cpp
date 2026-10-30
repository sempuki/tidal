// tidal-rules: SW-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct S {
  operator int() const { return 1; }
  operator int() { return 2; }
};
int f(S s) {
  switch (s) { case 2: return 0; default: return 1; }
}
