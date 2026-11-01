// tidal-rules: INST-36
// tidal-expect: run
// tidal-note: Each specialization of count has its own static variable, and so does each specialization of the generic lambda's call operator. Each specialization of make has its own closure type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> auto make() { return [] {}; }
static_assert(!same<decltype(make<int>()), decltype(make<long>())>);
static_assert(same<decltype(make<int>()), decltype(make<int>())>);
template <class T> int count() { static int k = 0; return ++k; }
auto bump = []<int I>() { static int k = 0; return ++k; };
int main() {
  count<int>();
  count<int>();
  int a = count<int>();
  int b = count<long>();
  bump.operator()<0>();
  int c = bump.operator()<0>();
  int d = bump.operator()<1>();
  return a == 3 && b == 1 && c == 2 && d == 1 ? 0 : 1;
}
