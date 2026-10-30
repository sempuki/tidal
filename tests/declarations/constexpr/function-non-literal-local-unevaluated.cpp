// tidal-rules: CX-3
// tidal-expect: accept
struct S { ~S() {} };
constexpr int f(bool b) {
  if (b) return 1;
  S s;
  return 2;
}
static_assert(f(true) == 1);
