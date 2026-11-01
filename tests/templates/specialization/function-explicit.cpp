// tidal-rules: TSP-2
// tidal-expect: accept
// tidal-note: The explicit specialization gives its own body. The argument list can be written or left to deduction.
template <class T> constexpr int code(T) { return 1; }
template <> constexpr int code<int>(int) { return 2; }
template <> constexpr int code(char) { return 3; }
static_assert(code(1.0) == 1);
static_assert(code(1) == 2);
static_assert(code('a') == 3);
