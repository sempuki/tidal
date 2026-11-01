// tidal-rules: PACK-18
// tidal-expect: accept
// tidal-note: The init capture c{p...} has no ellipsis before c, so c is one capture and not a pack. With one element, c is an int, and the body uses c without an ellipsis.
template <class... T> constexpr int only(T... p) { return [c{p...}] { return c; }(); }
static_assert(only(4) == 4);
