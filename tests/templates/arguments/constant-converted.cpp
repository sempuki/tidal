// tidal-rules: TARG-12
// tidal-expect: accept
// tidal-note: Each argument converts to the parameter type by a conversion that does not narrow, including a user-defined conversion in a constant expression.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
enum Plain { one = 1 };
struct Wrap { int v; constexpr operator int() const { return v; } };
constexpr Wrap seven{7};
template <int N> struct Num { static constexpr int value = N; };
template <long N> struct Wide { static constexpr long value = N; };
static_assert(Num<'a'>::value == 97);
static_assert(Num<one>::value == 1);
static_assert(Num<seven>::value == 7);
static_assert(Wide<3>::value == 3L);
static_assert(same<Num<'a'>, Num<97>>);
