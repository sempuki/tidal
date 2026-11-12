struct Range { int lo, hi; };
constexpr Range span() { return {2, 9}; }
int main() {
  if (auto [lo, hi] = span(); lo < hi) return hi - lo - 7;
  return 1;
}
