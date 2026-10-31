// tidal-rules: DTOR-7
// tidal-expect: reject
// tidal-error-gcc: accessing .s. outside its lifetime
struct S { int v = 1; constexpr ~S() {} };
constexpr int f() {
  S s;
  s.~S();
  return s.v;
}
static_assert(f() == 1);
