// tidal-rules: REF-5
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-error-clang: read of object outside its lifetime
// tidal-error-edg: attempt to access expired storage
constexpr const int& pass(const int& v) { return v; }
constexpr int f() {
  const int& r = pass(1);
  return r;
}
static_assert(f() == 1);
