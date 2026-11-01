// tidal-rules: PACK-9
// tidal-expect: accept
// tidal-note: With no elements, int(a...) is int(), which gives zero; Point{a...} is Point{}; and a base list expansion gives no bases.
template <class... A> constexpr int make(A... a) { return int(a...); }
static_assert(make() == 0);
static_assert(make(5) == 5);
struct Point { int x = 3; };
template <class... A> constexpr int build(A... a) { return Point{a...}.x; }
static_assert(build() == 3);
template <class... A> constexpr int count(A...) { return sizeof...(A); }
template <class... A> constexpr int forward_none(A... a) { return count(a...); }
static_assert(forward_none() == 0);
template <class... B> struct Derived : B... {};
struct Empty {};
static_assert(sizeof(Derived<>) == sizeof(Empty));
