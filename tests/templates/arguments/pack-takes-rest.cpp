// tidal-rules: TARG-21
// tidal-expect: accept
// tidal-note: The pack takes every argument after the fixed parameters, also none.
template <class First, class... Rest> struct Count { static constexpr int rest = sizeof...(Rest); };
static_assert(Count<int>::rest == 0);
static_assert(Count<int, char, long>::rest == 2);
