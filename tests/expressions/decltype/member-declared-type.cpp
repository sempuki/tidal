// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int m; const double d = 0; int& ref; };
void f(const S& s, S* p) {
  static_assert(same<decltype(s.m), int>);
  static_assert(same<decltype(p->d), const double>);
  static_assert(same<decltype(s.ref), int&>);
}
