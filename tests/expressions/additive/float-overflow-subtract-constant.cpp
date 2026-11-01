// tidal-rules: ADD-17
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: floating-point error
// tidal-note: Q-0001. Clang folds the difference to negative infinity. GCC and EDG reject it.
constexpr float lowest = -3.0e38f;
constexpr float x = lowest - 3.0e38f;
