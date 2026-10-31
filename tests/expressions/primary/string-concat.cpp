// tidal-rules: PRIM-7
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(sizeof("ab" "cd") == 5);
static_assert(sizeof(u8"ab" "cd") == 5);
static_assert(sizeof("ab" u8"cd") == 5);
static_assert(same<decltype(L"a" "b"), const wchar_t(&)[3]>);
