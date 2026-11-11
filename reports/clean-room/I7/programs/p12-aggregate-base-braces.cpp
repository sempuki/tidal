// CTAD-9, CTAD-10: a dependent base keeps its braces; a pack before a member is empty.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Base { T v; };
template <class T> struct Wrap : Base<T> { T w; };
template <class... B> struct Tail : B... { int x; };
int main() {
  Wrap x{{1}, 2};
  Tail t{5};
  static_assert(same<decltype(x), Wrap<int>> && same<decltype(t), Tail<>>);
  return x.v + x.w + t.x == 8 ? 0 : 1;
}
