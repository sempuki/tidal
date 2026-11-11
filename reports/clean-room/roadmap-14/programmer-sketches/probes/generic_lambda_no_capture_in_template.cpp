template <class T> constexpr int twice(T k) {
  auto g = []<class U>(U x) { return x * 2; };
  return g(k);
}
static_assert(twice(3) == 6);
int main() {}
