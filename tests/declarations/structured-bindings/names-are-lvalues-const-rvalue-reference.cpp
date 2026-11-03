// tidal-rules: SB-12
// tidal-expect: accept
// tidal-note: A class prvalue bound by copy, and by a const rvalue reference. Each name is an lvalue.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct P { int a; long b; };
auto [a, b] = P{1, 2};
const auto&& [c, d] = P{3, 4};
static_assert(same<decltype((a)), int&>);
static_assert(same<decltype((b)), long&>);
static_assert(same<decltype((c)), const int&>);
static_assert(same<decltype((d)), const long&>);
