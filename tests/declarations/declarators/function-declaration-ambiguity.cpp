// tidal-rules: DCL-13
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct W { int v; };
void f() {
  W w(W());
  w.v = 1;
}
