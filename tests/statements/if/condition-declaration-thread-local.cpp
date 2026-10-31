// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: does not allow storage class
// tidal-error-gcc: thread_local. invalid in condition
// tidal-error-edg: storage class may not be specified here
void f() {
  if (thread_local int x = 1) {}
}
