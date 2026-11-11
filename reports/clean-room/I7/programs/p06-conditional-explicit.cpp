// CTAD-5, CTAD-17: a conditional explicit evaluated for the deduced arguments.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Small { T v; constexpr explicit(sizeof(T) > 2) Small(T x) : v(x) {} };
int main() {
  Small a = 'c';
  Small b{100000};
  Small c = {short(3)};
  static_assert(same<decltype(a), Small<char>> && same<decltype(b), Small<int>> && same<decltype(c), Small<short>>);
  return a.v == 'c' && b.v == 100000 && c.v == 3 ? 0 : 1;
}
