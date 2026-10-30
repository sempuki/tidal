// tidal-rules: NEW-5
// tidal-expect: reject
constexpr bool f() {
  int* p = new int;
  delete[] p;
  return true;
}
static_assert(f());
