// tidal-rules: PACK-3
// tidal-expect: accept
// tidal-note: The named parameter x is one parameter of a placeholder type, and the ellipsis after it takes the extra arguments. The unnamed auto... is a pack.
constexpr int first_size(auto x...) { return sizeof(x); }
static_assert(first_size('a', 1, 2.0) == 1);
static_assert(first_size(1L) == sizeof(long));
constexpr int count(auto... rest) { return sizeof...(rest); }
constexpr int unnamed(auto...) { return 3; }
static_assert(count('a', 1, 2.0) == 3 && unnamed('a', 1) == 3);
