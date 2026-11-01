// tidal-rules: TSP-12
// tidal-expect: reject
// tidal-error-clang: cannot declare an explicit specialization in a friend
// tidal-error-gcc: explicit specialization in non-namespace scope
// tidal-error-edg: an explicit specialization declaration may not be a friend
template <class T> int size(T) { return 0; }
struct Host { template <> friend int size(char); };
