// tidal-rules: CTAD-25
// tidal-expect: accept
// tidal-note: The chosen guide gives Wide<long long>, and the initialization of Wide<long long> then converts the int argument.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Wide { T value; constexpr Wide(T v) : value(v) {} };
template <class T> Wide(T) -> Wide<long long>;
constexpr Wide w(2);
static_assert(same<decltype(w), const Wide<long long>> && w.value == 2);
