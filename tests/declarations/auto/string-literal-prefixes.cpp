// tidal-rules: AUTO-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto a = u8"x";
auto b = L"x";
auto c = u"x";
auto d = U"x";
static_assert(same<decltype(a), const char8_t*>);
static_assert(same<decltype(b), const wchar_t*>);
static_assert(same<decltype(c), const char16_t*>);
static_assert(same<decltype(d), const char32_t*>);
