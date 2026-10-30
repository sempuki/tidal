// tidal-rules: REF-5
// tidal-expect: reject
constexpr const int& local() {
  int x = 1;
  return x;
}
static_assert(local() == 1);
