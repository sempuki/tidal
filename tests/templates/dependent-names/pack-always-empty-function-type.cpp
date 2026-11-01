// tidal-rules: DEP-33
// tidal-expect: accept
// tidal-note: int(T) is a function type, and sizeof of a function type is not valid for any T. So every valid specialization of count has an empty pack. DEP-33 needs no diagnostic.
template <class... T> constexpr int count() { return (0 + ... + int(sizeof(int(T)))); }
static_assert(count<>() == 0);
