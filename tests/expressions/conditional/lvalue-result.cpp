// tidal-rules: COND-3
// tidal-expect: run
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int main() {
  int x = 1, y = 2;
  (true ? x : y) = 10;
  static_assert(same<decltype(true ? x : y), int&>);
  return x == 10 && y == 2 ? 0 : 1;
}
