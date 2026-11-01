// tidal-rules: NEW-37 NEW-28
// tidal-expect: accept
struct alignas(4 * __STDCPP_DEFAULT_NEW_ALIGNMENT__) Wide { int value; };
constexpr int make() {
  Wide* one = new Wide{7};
  Wide* many = new Wide[2]{{1}, {2}};
  int v = one->value + many[1].value;
  delete one;
  delete[] many;
  return v;
}
static_assert(make() == 9);
