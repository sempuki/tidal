// tidal-rules: LAM-1 AUTO-10
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction lambda_trailing_decltype_auto.cpp. A lambda with a trailing return type of decltype(auto), and a generic mutable lambda with the same trailing return type. Each returns its parameter by value.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int main() {
  auto plain = [](int x) -> decltype(auto) { return x; };
  auto counter = [n = 0](auto x) mutable -> decltype(auto) { ++n; return x + n; };
  static_assert(same<decltype(plain(1)), int>);
  static_assert(same<decltype(counter(1L)), long>);
  return plain(3) + counter(2) == 6 ? 0 : 1;
}
