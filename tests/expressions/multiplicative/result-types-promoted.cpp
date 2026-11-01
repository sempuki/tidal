// tidal-rules: MUL-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Small { one = 1 };
bool t = true;
unsigned char uc = 2;
unsigned u = 3;
long long ll = 4;
double d = 5;
static_assert(same<decltype(t * t), int>);
static_assert(same<decltype(uc % uc), int>);
static_assert(same<decltype(uc * u), unsigned>);
static_assert(same<decltype(one * 2), int>);
static_assert(same<decltype(u / ll), long long>);
static_assert(same<decltype(ll * d), double>);
static_assert(true * true == 1);
