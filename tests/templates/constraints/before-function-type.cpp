// tidal-rules: CNS-11
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-note: The constraint fails, so the return type, which instantiates Trap<int>, is never formed. Clang 18 forms the return type first (Q-0261).
template <class T> struct Trap { static_assert(sizeof(T) == 0); using type = int; };
template <class T> requires (sizeof(T) > 100) typename Trap<T>::type get(T);
constexpr int get(...) { return 2; }
static_assert(get(1) == 2);
