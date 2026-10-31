// tidal-rules: BF-6
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S { unsigned long x : 32; long y : 31; };
S s{};
static_assert(same<decltype(+s.x), unsigned int>);
static_assert(same<decltype(+s.y), int>);
