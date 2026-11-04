// tidal-rules: VC-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int m; };
void f(S& s, int S::* pm) { static_assert(same<decltype(static_cast<S&&>(s).*pm), int&&>); }
