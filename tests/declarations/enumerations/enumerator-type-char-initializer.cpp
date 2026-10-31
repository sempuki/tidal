// tidal-rules: ENUM-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Code { letter = 'x', check = same<decltype(letter), char> ? 1 : 0 };
static_assert(check == 1);
