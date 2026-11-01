// tidal-rules: PACK-1
// tidal-expect: accept
// tidal-note: The pattern T (&...rows)[2] declares a pack of references to arrays. An unnamed pack T... at the end of the list is a pack too.
template <class... T> constexpr int rows(T (&... row)[2]) { return (row[1] + ... + 0); }
template <class... T> constexpr int tail(int first, T...) { return first + int(sizeof...(T)); }
constexpr int a[2] = {1, 2};
constexpr int b[2] = {3, 4};
static_assert(rows(a, b) == 6);
static_assert(tail(10) == 10 && tail(10, 'c', 2.0) == 12);
