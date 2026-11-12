template <class T> constexpr int scale(T k) {
  auto g = [&](auto x) { return k * x; };
  return g(3);
}
static_assert(scale(2) == 6);
int main() {}
