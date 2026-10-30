// tidal-rules: DTOR-7
// tidal-expect: reject
struct S { int v = 1; constexpr ~S() {} };
constexpr int f() {
  S* p = new S;
  p->~S();
  int r = p->v;
  ::operator delete(p);
  return r;
}
static_assert(f() == 1);
