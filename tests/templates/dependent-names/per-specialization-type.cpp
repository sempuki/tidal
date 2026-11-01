// tidal-rules: DEP-7
// tidal-expect: accept
// tidal-note: T::type is looked up again for each specialization. Small and Large give different types.
struct Small { using type = char; };
struct Large { using type = long long; };
template <class T> constexpr int width() { return sizeof(typename T::type); }
static_assert(width<Small>() == 1);
static_assert(width<Large>() == sizeof(long long));
