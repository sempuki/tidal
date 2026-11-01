// tidal-rules: INIT-2
// tidal-expect: run
struct Bits {
  unsigned words[4];
  constexpr Bits() : words{} {}
};
constexpr bool all_zero(const Bits& b) {
  for (unsigned w : b.words) {
    if (w != 0) return false;
  }
  return true;
}
static_assert(all_zero(Bits()));
int main() {
  Bits b;
  int g[4] = {};
  if (!all_zero(b)) return 1;
  for (int v : g) {
    if (v != 0) return 2;
  }
  return 0;
}
