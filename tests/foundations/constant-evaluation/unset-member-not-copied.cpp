// tidal-rules: CE-22
// tidal-expect: accept
struct Pair {
  int first;
  int second;
  constexpr Pair() : first(1) {}
};
constexpr int read_first() {
  Pair original;
  return original.first;
}
static_assert(read_first() == 1);
