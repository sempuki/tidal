int main() {
  auto larger = []<class T>(T a, T b) { return a < b ? b : a; };
  static_assert(larger(3, 9) == 9);
  return larger(2.5, 1.5) == 2.5 ? 0 : 1;
}
