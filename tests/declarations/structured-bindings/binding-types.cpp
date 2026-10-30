// tidal-rules: SB-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { int a; const double b; };
S s{1, 2.0};
auto [p, q] = s;
auto& [r, t] = s;
const auto [u, v] = s;
static_assert(same<decltype(p), int>);
static_assert(same<decltype(q), const double>);
static_assert(same<decltype(r), int>);
static_assert(same<decltype(u), const int>);
