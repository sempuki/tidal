// tidal-rules: SW-2
// tidal-expect: reject
// tidal-error-clang: multiple conversions from switch condition type
// tidal-error-gcc: ambiguous default type conversion
// tidal-error-edg: more than one conversion function
struct S { operator int() const { return 1; } operator long() const { return 2; } };
void f(S s) {
  switch (s) { case 1: break; }
}
