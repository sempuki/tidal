// tidal-rules: MUL-12
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-note: Q-0001. Clang folds to infinity, and GCC and EDG reject. Tidal keeps the rejection while Q-0001 is open, because MUL-12 makes the overflow undefined and constant evaluation rejects an operation with undefined behavior (CE-9).
// tidal-error-gcc: 1\.0e\+308 \* 1\.0e\+1
// tidal-error-edg: floating-point error
constexpr double m(double a, double b) { return a * b; }
constexpr double big = m(1e308, 10.0);
