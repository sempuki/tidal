// tidal-rules: REF-5
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-error-clang: read of variable whose lifetime has ended
// tidal-error-edg: attempt to access expired storage
constexpr const int& local() {
  int x = 1;
  return x;
}
static_assert(local() == 1);
