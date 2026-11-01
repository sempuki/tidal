// tidal-rules: TARG-21
// tidal-expect: accept
// tidal-note: A constant pack and a template template pack take every argument after the fixed parameter, also none.
template <class T> struct One {};
template <int First, int... Rest> struct Values { static constexpr int rest = sizeof...(Rest); };
template <class T, template <class> class... Wraps> struct Wrapped { static constexpr int count = sizeof...(Wraps); };
static_assert(Values<1>::rest == 0 && Values<1, 2, 3>::rest == 2);
static_assert(Wrapped<int>::count == 0 && Wrapped<int, One, One>::count == 2);
