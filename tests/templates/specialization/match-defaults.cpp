// tidal-rules: TSP-14
// tidal-expect: accept
// tidal-note: The default of the template is part of the argument list that the partial specialization must match.
template <class T, class U = int> struct Pair { static constexpr int value = 0; };
template <class T> struct Pair<T, int> { static constexpr int value = 1; };
static_assert(Pair<char>::value == 1);
static_assert(Pair<char, long>::value == 0);
