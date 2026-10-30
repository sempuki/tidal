// tidal-rules: NEW-6
// tidal-expect: reject
constexpr bool f() {
  int* p = new int;
  return p != nullptr;
}
static_assert(f());
