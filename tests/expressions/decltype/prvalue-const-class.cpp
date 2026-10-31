// tidal-rules: DTY-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S {};
const S make();
const int cint();
static_assert(same<decltype(make()), const S>);
static_assert(same<decltype(cint()), int>);
