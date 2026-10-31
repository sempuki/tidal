// tidal-rules: OVL-19
// tidal-expect: accept
// tidal-note: Both templates give a specialization of type int(int*). The one from the more specialized template remains.
template <class T> constexpr int pick(T) { return 1; }
template <class T> constexpr int pick(T*) { return 2; }
constexpr int (*p)(int*) = pick;
static_assert(p(nullptr) == 2);
