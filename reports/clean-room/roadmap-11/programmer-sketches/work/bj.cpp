template <class K> constexpr int apply(K k) {
  auto f = [k](auto r) { return r * k; };
  return f(3);
}
static_assert(apply(2) == 6);
int main() {}
