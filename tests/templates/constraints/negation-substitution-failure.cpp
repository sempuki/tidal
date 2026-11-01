// tidal-rules: CNS-8
// tidal-expect: accept
// tidal-note: int::value cannot be formed, so (!T::value) is not satisfied, and the template is not viable.
template <class T> requires (!T::value) constexpr int f(T) { return 1; }
constexpr int f(...) { return 2; }
static_assert(f(1) == 2);
