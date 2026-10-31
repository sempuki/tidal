// tidal-rules: SW-3
// tidal-expect: reject
// tidal-error-clang: multiple default labels in one switch
// tidal-error-gcc: multiple default labels in one switch
// tidal-error-edg: default label has already appeared
void f(int v) {
  switch (v) { default: break; default: break; }
}
