// tidal-rules: COMMA-2
// tidal-expect: run
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int main() {
  int x = 1, y = 2;
  static_assert(same<decltype((x, y)), int&>);
  static_assert(same<decltype((x, 3.0)), double>);
  (x, y) = 9;
  return y == 9 && x == 1 ? 0 : 1;
}
