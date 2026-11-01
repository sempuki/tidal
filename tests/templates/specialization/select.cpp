// tidal-rules: TSP-13
// tidal-expect: accept
// tidal-note: An explicit specialization wins over a partial specialization that also matches, and the template applies when nothing matches.
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> struct Kind<T*> { static constexpr int value = 1; };
template <> struct Kind<int*> { static constexpr int value = 2; };
static_assert(Kind<int>::value == 0);
static_assert(Kind<char*>::value == 1);
static_assert(Kind<int*>::value == 2);
