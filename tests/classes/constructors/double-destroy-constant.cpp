// tidal-rules: DTOR-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0054. EDG 7.0 accepts a second destruction of an object during constant evaluation.
struct S { int v = 1; constexpr ~S() {} };
constexpr int f() {
  S s;
  s.~S();
  return 0;
}
static_assert(f() == 0);
