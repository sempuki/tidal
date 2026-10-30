// tidal-rules: SB-4
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int g = 0;
struct R { int& r; };
R rr{g};
auto [x] = rr;
static_assert(same<decltype(x), int&>);
