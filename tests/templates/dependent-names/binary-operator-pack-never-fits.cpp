// tidal-rules: DEP-37
// tidal-expect: reject
// tidal-error-clang: must be a binary operator
// tidal-error-gcc: must have exactly two arguments
// tidal-error-edg: too many parameters for this operator
// tidal-note: Only an empty pack gives the operator two parameters.
struct Item {};
template <class... T> bool operator<(Item, Item, T...);
