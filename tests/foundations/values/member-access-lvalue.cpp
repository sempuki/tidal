// tidal-rules: VC-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int m; };
void f(S& s, S* p) {
  static_assert(same<decltype((s.m)), int&>);
  static_assert(same<decltype((p->m)), int&>);
  int* a = &s.m; int* b = &p->m; (void)a; (void)b;
}
