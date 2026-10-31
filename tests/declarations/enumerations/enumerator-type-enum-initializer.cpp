// tidal-rules: ENUM-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Old { first = 1 };
enum New { copy = first, check = same<decltype(copy), Old> ? 1 : 0 };
static_assert(check == 1);
