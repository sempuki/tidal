// tidal-rules: MUL-12
// tidal-expect-clang: accept
// tidal-expect-gcc: reject
// tidal-note: Q-0001. Clang folds to infinity, GCC rejects.
// tidal-expect-edg: reject
// tidal-error-gcc: 1\.0e\+308 \* 1\.0e\+1
// tidal-error-edg: floating-point error
constexpr double m(double a, double b) { return a * b; }
constexpr double big = m(1e308, 10.0);
