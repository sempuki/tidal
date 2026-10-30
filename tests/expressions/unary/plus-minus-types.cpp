// tidal-rules: UN-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
char c = 1;
short s = 1;
enum E { A = 2 };
unsigned u = 1;
double d = 1;
static_assert(same<decltype(+c), int>);
static_assert(same<decltype(-s), int>);
static_assert(same<decltype(+A), int>);
static_assert(same<decltype(-u), unsigned>);
static_assert(same<decltype(-d), double>);
int arr[2];
static_assert(same<decltype(+arr), int*>);
