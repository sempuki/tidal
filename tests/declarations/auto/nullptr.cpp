// tidal-rules: AUTO-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto n = nullptr;
static_assert(same<decltype(n), decltype(nullptr)>);
