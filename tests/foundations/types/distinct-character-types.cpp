// tidal-rules: TYP-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(!same<char, signed char> && !same<char, unsigned char>);
static_assert(!same<wchar_t, int> && !same<char8_t, unsigned char>);
static_assert(!same<char16_t, unsigned short> && !same<char32_t, unsigned>);
