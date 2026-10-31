// tidal-rules: INIT-1
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: accessing uninitialized array element
// tidal-error-clang: read of uninitialized object
// tidal-note: Q-0013. EDG 7.0 accepts reading an uninitialized array element during constant evaluation.
constexpr int f() {
  int a[2];
  a[0] = 1;
  return a[1];
}
static_assert(f() == 0);
