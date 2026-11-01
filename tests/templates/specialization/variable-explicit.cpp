// tidal-rules: TSP-2
// tidal-expect: accept
template <class T> constexpr int rank = 0;
template <> constexpr int rank<int> = 1;
template <> constexpr int rank<long> = 2;
static_assert(rank<char> == 0);
static_assert(rank<int> == 1);
static_assert(rank<long> == 2);
