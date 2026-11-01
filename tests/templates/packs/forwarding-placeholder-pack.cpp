// tidal-rules: PACK-22 PACK-23
// tidal-expect: accept
// tidal-note: auto&&... x is a pack of forwarding references with no named type pack. static_cast<decltype(x)>(x) gives back each argument with its value category.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
constexpr auto kinds = [](auto&&... x) { return Tag<void(decltype((static_cast<decltype(x)>(x)))...)>{}; };
int n = 0;
static_assert(same<decltype(kinds(n, 1, static_cast<int&&>(n))), Tag<void(int&, int&&, int&&)>>);
