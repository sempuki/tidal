// tidal-rules: CONV-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Plain { A = 1 };
enum Small : unsigned char { B = 1 };
enum Big : unsigned long long { C = 1 };
static_assert(same<decltype(+A), int>);
static_assert(same<decltype(+B), int>);
static_assert(same<decltype(+C), unsigned long long>);
char32_t c32 = 0;
static_assert(same<decltype(+c32), unsigned int>);
