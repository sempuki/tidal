// tidal-rules: RFOR-5
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: loop variable .v. may not be declared .thread_local
// tidal-error-gcc: for-range-declaration cannot be .thread_local
// tidal-error-edg: thread-local storage class is not valid here
// tidal-note: Q-0026. GCC 14 accepts a storage class on a range-based for variable; GCC 16, Clang, and EDG reject it.
void f() {
  int a[2] = {1, 2};
  for (thread_local int v : a) (void)v;
}
