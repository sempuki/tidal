// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: Gap review RV6-B-3. An enumeration can be the qualifier of a qualified name in substitution. T::red is valid when the enumeration has the enumerator red, scoped or not, and the template is chosen. Without the enumerator, or with a T that is neither a class nor an enumeration, substitution fails and the fallback is chosen.
enum Color { red };
enum class Signal { red };
enum Shade { blue };
template <class T> constexpr int pick(int, decltype(T::red)* = nullptr) { return 1; }
template <class T> constexpr int pick(long) { return 2; }
static_assert(pick<Color>(0) == 1);
static_assert(pick<Signal>(0) == 1);
static_assert(pick<Shade>(0) == 2);
static_assert(pick<int>(0) == 2);
