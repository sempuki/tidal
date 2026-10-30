// tidal-rules: AUTO-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
const int ci = 1;
volatile int vi = 2;
int i = 3;
int& ri = i;
const int& cri = i;
auto a = ci;
auto b = vi;
auto c = ri;
auto d = cri;
static_assert(same<decltype(a), int>);
static_assert(same<decltype(b), int>);
static_assert(same<decltype(c), int>);
static_assert(same<decltype(d), int>);
