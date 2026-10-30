// tidal-rules: NEW-4
// tidal-expect: accept
constexpr bool f() {
  int* p = nullptr;
  delete p;
  return true;
}
static_assert(f());
