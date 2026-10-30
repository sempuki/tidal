// tidal-rules: SW-2
// tidal-expect: reject
struct S { operator int() const { return 1; } operator long() const { return 2; } };
void f(S s) {
  switch (s) { case 1: break; }
}
