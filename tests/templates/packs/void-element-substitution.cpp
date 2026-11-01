// tidal-rules: PACK-27 PACK-28
// tidal-expect: accept
// tidal-note: f<void> gives the first template a parameter of type void, so substitution fails, and the second template is chosen.
template <class... T> constexpr int f(T...) { return 1; }
template <class T> constexpr int f(...) { return 2; }
static_assert(f<void>() == 2);
static_assert(f<int>(1) == 1);
