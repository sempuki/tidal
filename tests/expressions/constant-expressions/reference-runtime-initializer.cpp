// tidal-rules: CE-29
// tidal-expect: reject
// tidal-error-clang: must be initialized by a constant expression
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: must have a constant value
// tidal-note: The initializer of later calls a function that is not constexpr, so later is not usable in constant expressions.
int a = 0;
int& pick() { return a; }
int& later = pick();
constexpr int* q = &later;
