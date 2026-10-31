// tidal-rules: ENUM-10
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum class Scoped { a = 4, b = same<decltype(a), int> ? 1 : 2 };
static_assert(static_cast<int>(Scoped::b) == 1);
enum Fixed : long { f1 = 1, f2 = same<decltype(f1), long> ? 1 : 2 };
static_assert(f2 == 1);
enum Plain { p1 = 1u, p2 = same<decltype(p1), unsigned> ? 1 : 2, p3 };
static_assert(p2 == 1);
static_assert(same<decltype(p1), Plain>);
