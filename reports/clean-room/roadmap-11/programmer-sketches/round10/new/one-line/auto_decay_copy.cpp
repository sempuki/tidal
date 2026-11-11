// auto(x) makes a prvalue copy (C++23), and the uz literal suffix gives the size type.
struct Count { int n; };
int bump(Count&& c) { return ++c.n; }
int main() {
  Count c{1};
  int r = bump(auto(c));
  auto z = 3uz;
  static_assert(sizeof(z) == sizeof(sizeof 0));
  return r == 2 && c.n == 1 && z == 3 ? 0 : 1;
}
