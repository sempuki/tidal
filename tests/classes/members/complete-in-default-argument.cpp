// tidal-rules: CLS-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct S {
  int f(int a = later) { return a; }
  static const int later = 3;
};
