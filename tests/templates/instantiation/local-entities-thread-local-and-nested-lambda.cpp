// tidal-rules: INST-36
// tidal-expect: run
// tidal-note: Each specialization of tick has its own thread_local variable. Each specialization of the generic lambda's call operator gives the inner lambda its own closure type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> int tick() { thread_local int k = 0; return ++k; }
auto outer = [](auto v) { return [] {}; };
static_assert(!same<decltype(outer(1)), decltype(outer(1L))>);
static_assert(same<decltype(outer(1)), decltype(outer(2))>);
int main() {
  tick<int>();
  int a = tick<int>();
  int b = tick<char>();
  return a == 2 && b == 1 ? 0 : 1;
}
