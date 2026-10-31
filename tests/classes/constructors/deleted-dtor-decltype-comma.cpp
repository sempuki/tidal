// tidal-rules: DTOR-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Pinned { Pinned() = default; ~Pinned() = delete; };
Pinned make();
static_assert(same<decltype(0, make()), Pinned>);
static_assert(same<decltype((make())), Pinned>);
