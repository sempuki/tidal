// tidal-rules: PRIM-7
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype("abc"), const char(&)[4]>);
static_assert(same<decltype(u8"abc"), const char8_t(&)[4]>);
static_assert(same<decltype(L"abc"), const wchar_t(&)[4]>);
static_assert(sizeof("abc") == 4);
static_assert(sizeof(u"ab") == 6);
