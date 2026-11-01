// tidal-rules: DEP-37
// tidal-expect: reject
// tidal-error-clang: must be a unary operator
// tidal-error-gcc: must have exactly one argument
// tidal-error-edg: too many parameters for this operator
struct Item {};
template <class... T> int operator~(Item, T...);
