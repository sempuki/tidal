// tidal-rules: COND-4
// tidal-expect: run
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x = 4;
int main() {
  static_assert(same<decltype(true ? x : throw 1), int&>);
  static_assert(same<decltype(false ? throw 1 : 2), int>);
  (true ? x : throw 1) = 5;
  return x == 5 ? 0 : 1;
}
