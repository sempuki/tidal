// CTAD-1: a new expression and functional casts.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Cell { T v; constexpr Cell(T x) : v(x) {} };
int main() {
  auto p = new Cell(2.5);
  static_assert(same<decltype(p), Cell<double>*>);
  auto q = Cell('z');
  auto r = Cell{7u};
  static_assert(same<decltype(q), Cell<char>> && same<decltype(r), Cell<unsigned>>);
  int ok = p->v == 2.5 && q.v == 'z' && r.v == 7u;
  delete p;
  return ok ? 0 : 1;
}
