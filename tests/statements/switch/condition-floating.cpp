// tidal-rules: SW-2
// tidal-expect: reject
// tidal-error-clang: requires expression of integer type .*double. invalid
// tidal-error-gcc: switch quantity not an integer
// tidal-error-edg: must have integral or enum type
void f(double d) {
  switch (d) { case 1: break; }
}
