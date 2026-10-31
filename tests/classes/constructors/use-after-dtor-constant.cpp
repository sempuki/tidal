// tidal-rules: DTOR-7
// tidal-expect: reject
// tidal-error-gcc: accessing .s. outside its lifetime
// tidal-error-clang: read of object outside its lifetime
// tidal-error-edg: access to uninitialized object
struct S { int v = 1; constexpr ~S() {} };
constexpr int f() {
  S s;
  s.~S();
  return s.v;
}
static_assert(f() == 1);
