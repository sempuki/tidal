// tidal-rules: REF-5
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
constexpr const int& local() {
  int x = 1;
  return x;
}
static_assert(local() == 1);
