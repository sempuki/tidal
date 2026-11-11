template <class F> struct Box { F f; };
template <class K> constexpr auto times(K k) {
  auto l = [k](int r) { return r * k; };
  return Box{l};
}
int main() { return times(2).f(3) == 6 ? 0 : 1; }
