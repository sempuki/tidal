// tidal-rules: DTOR-7 VC-10
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: outside its lifetime
// tidal-error-clang: destroying object .* whose lifetime has already ended
// tidal-note: Q-0054. EDG 7.0 accepts a second destruction of an object during constant evaluation.
struct S {
  int v;
  constexpr S(int x) : v(x) {}
  constexpr S(const S& o) : v(o.v) {}
  constexpr ~S() {}
};
constexpr void take(S s) { s.~S(); }  // the caller still ends s later
constexpr int f() {
  take(S(1));
  return 0;
}
static_assert(f() == 0);
