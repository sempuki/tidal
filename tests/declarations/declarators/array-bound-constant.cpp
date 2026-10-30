// tidal-rules: DCL-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr int n = 4;
enum { Five = 5 };
int a[n];
int b[Five];
int c['a' - 'a' + 1];
static_assert(same<decltype(a), int[4]>);
static_assert(same<decltype(b), int[5]>);
static_assert(same<decltype(c), int[1]>);
