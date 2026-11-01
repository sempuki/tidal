// tidal-rules: CPT-26
// tidal-expect: accept
struct Tagged { using tag = int; void run(); };
template <class T> constexpr bool ok = requires (T t) { t.run(); typename T::tag; };
static_assert(ok<Tagged> && !ok<int>);
template <class T> constexpr int pick() { if constexpr (requires (T t) { t.run(); }) return 1; else return 2; }
static_assert(pick<Tagged>() == 1 && pick<int>() == 2);
