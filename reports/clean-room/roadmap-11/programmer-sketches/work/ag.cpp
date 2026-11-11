template <class F> struct Box { F f; };
template <class K> constexpr auto times(K k) {
  auto b = Box{[k](int r) { return r * k; }};
  return b;
}
int main() { return times(2).f(3) == 6 ? 0 : 1; }
