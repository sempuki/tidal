// tidal-rules: CPT-11
// tidal-expect: accept
template <class T> auto first(T v) -> int requires (sizeof(T) > 1) { return v; }
template <class T> requires (sizeof(T) < 100) auto second(T v) -> int requires (sizeof(T) > 1) { return v; }
static_assert(sizeof(first(1)) > 0 && sizeof(second(2)) > 0);
