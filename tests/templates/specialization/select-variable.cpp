// tidal-rules: TSP-13
// tidal-expect: accept
// tidal-note: For the variable template, the explicit specialization wins over a matching partial specialization, and the primary template applies when nothing matches.
template <class T> constexpr int kind = 0;
template <class T> constexpr int kind<T*> = 1;
template <> constexpr int kind<int*> = 2;
static_assert(kind<int> == 0 && kind<char*> == 1 && kind<int*> == 2);
