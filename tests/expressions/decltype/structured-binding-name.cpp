// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct P { int x; double y; };
void g() { P p{}; auto& [a, b] = p; static_assert(same<decltype(a), int>); }
