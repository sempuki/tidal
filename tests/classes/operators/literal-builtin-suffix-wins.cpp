// tidal-rules: OPR-8
// tidal-expect: run
// tidal-note: The program declares literal operators for suffixes that are forms in the syntax of literals. Each literal with such a suffix keeps its built-in type and value, and no literal operator runs.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int calls = 0;
int operator"" u(unsigned long long) { return ++calls; }
int operator"" z(unsigned long long) { return ++calls; }
int operator"" ll(unsigned long long) { return ++calls; }
int operator"" f(long double) { return ++calls; }
int operator"" L(long double) { return ++calls; }
int main() {
  static_assert(same<decltype(5u), unsigned>);
  static_assert(sizeof(5z) == sizeof(decltype(sizeof 0)) && 5z - 6z < 0);
  static_assert(same<decltype(5ll), long long>);
  static_assert(same<decltype(5.0f), float>);
  static_assert(same<decltype(5.0L), long double>);
  unsigned a = 5u;
  long long b = 5ll;
  float c = 5.0f;
  if (a != 5 || b != 5 || c != 5.0f) return 1;
  return calls;
}
