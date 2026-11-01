// tidal-rules: TPL-30
// tidal-expect: accept
// tidal-note: A pack is the last parameter of a class template. A function template can declare a pack before other parameters.
template <class... Ts> struct List { static constexpr int count = sizeof...(Ts); };
template <int... Ns> struct Values { static constexpr int count = sizeof...(Ns); };
template <class... Ts, class Last> constexpr int last(Last) { return sizeof...(Ts); }
static_assert(List<>::count == 0);
static_assert(List<int, char>::count == 2);
static_assert(Values<1, 2, 3>::count == 3);
static_assert(last(1) == 0);
