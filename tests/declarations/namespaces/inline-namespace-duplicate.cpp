// tidal-rules: NS-3
// tidal-expect: reject
// tidal-error-clang: reference to .x. is ambiguous
// tidal-error-gcc: reference to .x. is ambiguous
// tidal-error-edg: A::x. is ambiguous
// tidal-note: Gap review 2026-10-30.
namespace A {
  inline namespace B { int x; }
  int x;
}
int y = A::x;
