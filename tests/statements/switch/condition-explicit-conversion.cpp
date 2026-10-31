// tidal-rules: SW-2
// tidal-expect: reject
// tidal-error-clang: requires explicit conversion to .int
// tidal-error-gcc: switch quantity not an integer
// tidal-error-edg: must have integral or enum type
struct S { explicit operator int() const { return 1; } };
void f(S s) {
  switch (s) { case 1: break; }
}
