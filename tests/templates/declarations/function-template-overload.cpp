// tidal-rules: TPL-22
// tidal-expect: accept
// tidal-note: A function template, a second template with another parameter list, and a plain function share one name.
template <class T> constexpr int which(T) { return 1; }
template <class T, class U> constexpr int which(T, U) { return 2; }
constexpr int which(double) { return 3; }
static_assert(which(1) == 1);
static_assert(which(1, 2) == 2);
static_assert(which(1.0) == 3);
