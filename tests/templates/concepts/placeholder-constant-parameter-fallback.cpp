// tidal-rules: CPT-21 CNS-11
// tidal-expect: accept
// tidal-note: The type constraint of N is a constraint of the first template. For 'a' the constraint fails, the first template is not viable, and the second is chosen.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide auto N> constexpr int pick() { return 1; }
template <auto N> constexpr int pick() { return 2; }
static_assert(pick<'a'>() == 2);
