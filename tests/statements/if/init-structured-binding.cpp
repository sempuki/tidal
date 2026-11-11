// tidal-rules: IF-7
// tidal-expect: run
struct Range { int low, high; };
int main() {
  Range r{2, 5};
  if (auto [low, high] = r; low < high) {
    if (high - low != 3) return 1;
  } else {
    return 2;
  }
  if (auto& [low, high] = r; low > 0) high = 9;
  if (r.high != 9) return 3;
  if constexpr (true) {
    if (const auto [low, high] = Range{1, 1}; low != high) return 4;
  }
  return 0;
}
