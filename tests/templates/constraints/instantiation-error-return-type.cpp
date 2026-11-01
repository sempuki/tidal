// tidal-rules: CNS-33
// tidal-expect: reject
// tidal-error: static assertion failed
// tidal-note: sizeof(make(T())) needs the return type of make<int>, which needs the body. The body fails, and the error is not a substitution failure.
template <class T> auto make(T) { static_assert(sizeof(T) == 0); return 1; }
template <class T> requires (sizeof(make(T())) > 0) constexpr int f(T) { return 1; }
constexpr int f(...) { return 2; }
static_assert(f(1) == 2);
