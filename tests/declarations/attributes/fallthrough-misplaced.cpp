// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: fallthrough annotation is outside switch statement
// tidal-error-gcc: invalid use of attribute .fallthrough
// tidal-error-edg: fallthrough. attribute may only appear in an enclosing switch
void f(int k) {
  [[fallthrough]];
  (void)k;
}
