// tidal-rules: PACK-3
// tidal-expect: accept
// tidal-note: T names no pack, so T... is a parameter of type T followed by the C variable argument part. A pack type or auto makes T... a pack.
template <class T> constexpr int variadic(T...) { return 1; }
static_assert(variadic(1, 2.0, "text") == 1);
template <class... T> constexpr int pack(T...) { return sizeof...(T); }
static_assert(pack(1, 2.0, "text") == 3);
constexpr int placeholder(auto...) { return 7; }
static_assert(placeholder(1, 2) == 7);
