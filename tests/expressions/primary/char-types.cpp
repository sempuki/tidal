// tidal-rules: PRIM-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype('a'), char>);
static_assert(same<decltype(L'a'), wchar_t>);
static_assert(same<decltype(u8'a'), char8_t>);
static_assert(same<decltype(u'a'), char16_t>);
static_assert(same<decltype(U'a'), char32_t>);
