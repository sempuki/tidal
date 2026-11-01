// tidal-rules: PACK-16
// tidal-expect: accept
// tidal-note: alignas with an expansion takes the strictest alignment of its elements. With no elements, the member keeps its own alignment.
struct alignas(2) Two { char c; };
struct alignas(8) Eight { char c; };
struct alignas(4) Four { char c; };
template <class... T> struct ByType { alignas(T...) char c; };
static_assert(alignof(ByType<Two, Eight, Four>) == 8);
static_assert(alignof(ByType<Eight, Two>) == 8);
static_assert(alignof(ByType<>) == 1);
template <int... N> struct ByValue { alignas(N...) char c; };
static_assert(alignof(ByValue<2, 16, 4>) == 16);
