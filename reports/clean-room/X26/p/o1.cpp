// Overloaded comma today: member, non-member, enumeration, template, call by name, fold.
struct M { int v; M operator,(M r) const { return {v * 10 + r.v}; } };
struct N { int v; };
N operator,(N l, N r) { return {l.v + r.v}; }
enum class E { a = 1, b = 2 };
int operator,(E l, E r) { return int(l) * 100 + int(r); }
template <class T> struct W { T v; };
template <class T> W<T> operator,(W<T> l, W<T> r) { return {l.v - r.v}; }
template <class... T> auto fold(T... t) { return (t, ...); }
int main() {
  M m = (M{1}, M{2});
  N n = (N{1}, N{2});
  int e = (E::a, E::b);
  W<int> w = (W<int>{9}, W<int>{4});
  M byname = M{3}.operator,(M{4});
  N f = fold(N{1}, N{2}, N{3});
  return m.v == 12 && n.v == 3 && e == 102 && w.v == 5 && byname.v == 34 && f.v == 6 ? 0 : 1;
}
