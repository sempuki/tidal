// tidal-rules: CX-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
void f() {
  static int a = 0;
  static constexpr int* p = &a;
  (void)p;
}
