// tidal-rules: DTY-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
int& r = i;
int&& rr = 1;
const int c = 2;
static_assert(same<decltype(i), int>);
static_assert(same<decltype(r), int&>);
static_assert(same<decltype(rr), int&&>);
static_assert(same<decltype(c), const int>);
