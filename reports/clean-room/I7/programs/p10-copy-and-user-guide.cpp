// CTAD-20, CTAD-21: the copy candidate, and a user guide that wraps.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { T v; constexpr Box(T x) : v(x) {} };
template <class T> struct Nest { T v; constexpr Nest(T x) : v(x) {} };
template <class T> Nest(Nest<T>) -> Nest<Nest<T>>;
int main() {
  Box a(1);
  Box b(a);
  Nest n(1);
  Nest m(n);
  static_assert(same<decltype(b), Box<int>> && same<decltype(m), Nest<Nest<int>>>);
  return b.v == 1 && m.v.v == 1 ? 0 : 1;
}
