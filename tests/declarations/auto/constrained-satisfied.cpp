// tidal-rules: AUTO-13
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> concept Int = same<T, int>;
Int auto a = 1;
const Int auto& b = a;
static_assert(same<decltype(a), int>);
static_assert(same<decltype(b), const int&>);
Int decltype(auto) c = 2;
static_assert(same<decltype(c), int>);
