// tidal-rules: NS-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
namespace A {
  inline namespace B { int x; }
  int x;
}
int y = A::x;
