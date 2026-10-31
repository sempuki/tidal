// tidal-rules: ADD-17
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: 1\.7976931348623157e\+308 \* 2\.0e\+0
// tidal-error-edg: floating-point error
// tidal-note: Q-0001. Clang folds to infinity. GCC and EDG reject it.
// tidal-note: Gap review 2026-10-31 (third).
constexpr double big = 1.7976931348623157e308;
constexpr double x = big + big;
