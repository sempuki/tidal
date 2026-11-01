// tidal-rules: DED-3
// tidal-expect: accept
// tidal-note: With T fixed, both parameters are long, and each argument converts, also by a conversion function.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct Meter { constexpr operator int() const { return 5; } };
template <class T> constexpr T add(T a, T b) { return a + b; }
static_assert(add<long>(1, 2L) == 3);
static_assert(same<decltype(add<long>(1, 'a')), long>);
static_assert(add<int>(Meter{}, 1) == 6);
