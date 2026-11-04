// tidal-rules: TR-3
// tidal-expect: reject
// tidal-error-clang: not allowed to be declared constexpr
// tidal-error-gcc: to be .constexpr.
// tidal-error-edg: may not be declared constexpr
constexpr int main() { return 0; }
