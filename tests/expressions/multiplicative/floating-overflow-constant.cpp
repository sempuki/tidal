// tidal-rules: MUL-12
// tidal-expect-clang: accept
// tidal-expect-gcc: reject
// tidal-note: Divergence MUL-D1. Clang folds to infinity, GCC rejects.
constexpr double m(double a, double b) { return a * b; }
constexpr double big = m(1e308, 10.0);
