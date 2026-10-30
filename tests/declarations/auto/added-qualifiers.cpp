// tidal-rules: AUTO-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int i = 0;
const auto a = i;
auto* p = &i;
const auto* q = &i;
auto* const r = &i;
volatile auto v = 1;
static_assert(same<decltype(a), const int>);
static_assert(same<decltype(p), int*>);
static_assert(same<decltype(q), const int*>);
static_assert(same<decltype(r), int* const>);
static_assert(same<decltype(v), volatile int>);
