// tidal-rules: MUL-12
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: floating-point error
// tidal-note: Q-0001. Clang folds the quotient to infinity. GCC and EDG reject it.
constexpr double q(double a, double b) { return a / b; }
constexpr double big = q(1e308, 1e-10);
