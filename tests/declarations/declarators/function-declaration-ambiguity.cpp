// tidal-rules: DCL-13
// tidal-expect: reject
// tidal-error-clang: is not a structure or union
// tidal-error-gcc: which is of non-class type
// tidal-error-edg: must have class type
// tidal-note: Gap review 2026-10-30.
struct W { int v; };
void f() {
  W w(W());
  w.v = 1;
}
