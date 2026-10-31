// tidal-rules: RFOR-5
// tidal-expect: reject
// tidal-error-clang: loop variable .v. may not be declared .extern
// tidal-error-gcc: for-range-declaration cannot be .extern|has both .extern. and initializer
// tidal-error-edg: storage class may not be specified here
// tidal-note: Gap review 2026-10-30.
void f() {
  int a[2] = {1, 2};
  for (extern int v : a) (void)v;
}
