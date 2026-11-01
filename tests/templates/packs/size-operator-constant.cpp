// tidal-rules: PACK-19
// tidal-expect: accept
// tidal-note: sizeof... of an empty pack is zero. The result is a constant, so it can be a template argument and an array bound.
template <unsigned long N> struct Count { static constexpr unsigned long value = N; };
template <class... T> using Counted = Count<sizeof...(T)>;
static_assert(Counted<>::value == 0 && Counted<int, char, long>::value == 3);
template <class... T> struct Table { char cells[sizeof...(T) + 1]; };
static_assert(sizeof(Table<int, int>) == 3);
template <class... T> constexpr auto none(T... x) { return sizeof...(x); }
static_assert(none() == 0);
