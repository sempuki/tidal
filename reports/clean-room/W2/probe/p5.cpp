constexpr const int& clamp(const int& v, const int& lo, const int& hi) {
  return v < lo ? lo : hi < v ? hi : v;
}
int main() {
  bool chain = 3 > 2 > 1;
  bool mix = -1 < 0u;
  const int& r = clamp(50, 0, 10);
  return r + chain + mix;
}
