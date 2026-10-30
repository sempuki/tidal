// tidal-rules: SW-2
// tidal-expect: reject
struct S { explicit operator int() const { return 1; } };
void f(S s) {
  switch (s) { case 1: break; }
}
