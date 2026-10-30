// tidal-rules: AUTO-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
const auto& r = 1;
static_assert(same<decltype(r), const int&>);
