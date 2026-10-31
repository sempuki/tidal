// tidal-rules: RFOR-5
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: loop variable .v. may not be declared .static
// tidal-error-gcc: for-range-declaration cannot be .static
// tidal-error-edg: storage class may not be specified here
// tidal-note: Q-0026. GCC 14 accepts a storage class on a range-based for variable; GCC 16, Clang, and EDG reject it.
void f() {
  int a[2] = {1, 2};
  for (static int v : a) (void)v;
}
