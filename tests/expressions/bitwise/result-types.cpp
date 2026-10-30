// tidal-rules: BIT-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
char c = 1;
unsigned u = 1;
long l = 1;
bool b = true;
unsigned char uc = 0;
enum Plain { A = 1 };
static_assert(same<decltype(c & c), int>);
static_assert(same<decltype(u & 1), unsigned>);
static_assert(same<decltype(l | 1), long>);
static_assert(same<decltype(b ^ b), int>);
static_assert(same<decltype(~uc), int>);
static_assert(same<decltype(A | A), int>);
