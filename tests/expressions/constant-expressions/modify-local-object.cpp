// tidal-rules: CE-5
// tidal-expect: accept
constexpr int f() {
  int counter = 0;
  ++counter;
  return counter;
}
static_assert(f() == 1);
