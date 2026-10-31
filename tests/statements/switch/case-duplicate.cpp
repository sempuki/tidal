// tidal-rules: SW-3
// tidal-expect: reject
// tidal-error-clang: duplicate case value
// tidal-error-gcc: duplicate case value
// tidal-error-edg: case label value has already appeared
void f(int v) {
  switch (v) { case 1: break; case 1: break; }
}
