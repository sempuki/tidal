// tidal-rules: CONV-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
bool b = true;
char c = 1;
signed char sc = 1;
unsigned char uc = 1;
short s = 1;
unsigned short us = 1;
char16_t c16 = 1;
wchar_t wc = 1;
static_assert(same<decltype(+b), int>);
static_assert(same<decltype(+c), int>);
static_assert(same<decltype(+sc), int>);
static_assert(same<decltype(+uc), int>);
static_assert(same<decltype(+s), int>);
static_assert(same<decltype(+us), int>);
static_assert(same<decltype(+c16), int>);
static_assert(sizeof(wchar_t) < sizeof(long long));
float f = 1;
static_assert(same<decltype(+f), float>);
