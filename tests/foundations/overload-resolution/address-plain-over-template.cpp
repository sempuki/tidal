// tidal-rules: OVL-19
// tidal-expect: accept
// tidal-note: The template gives the specialization pick<int> with the same type. The plain function remains and the specialization is removed.
constexpr int pick(int) { return 1; }
template <class T> constexpr int pick(T) { return 2; }
constexpr int (*p)(int) = pick;
static_assert(p(0) == 1);
