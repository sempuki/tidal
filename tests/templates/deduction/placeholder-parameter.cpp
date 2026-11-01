// tidal-rules: DED-34
// tidal-expect: accept
// tidal-note: Each auto parameter adds an invented type parameter after those of the template head.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
constexpr auto twice(auto x) { return x + x; }
static_assert(twice(2) == 4);
static_assert(same<decltype(twice(1.5)), double>);
template <class T> auto mix(T, auto u) { return Tag<void(T, decltype(u))>{}; }
static_assert(same<decltype(mix<int, char>(1, 'c')), Tag<void(int, char)>>);
static_assert(same<decltype(mix<long>(1, 'c')), Tag<void(long, char)>>);
auto both(auto a, const auto& b) { return Tag<void(decltype(a), decltype(b))>{}; }
static_assert(same<decltype(both<int, long>(1, 2L)), Tag<void(int, const long&)>>);
