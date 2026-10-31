// tidal-rules: ENUM-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr short small = 3;
enum Size { count = small, check = same<decltype(count), short> ? 1 : 0 };
static_assert(check == 1);
