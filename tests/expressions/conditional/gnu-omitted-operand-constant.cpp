// tidal-rules: COND-14
// tidal-expect: reject
// tidal-error-clang: omitting middle operand
// tidal-error-gcc: omitted middle operand
// tidal-error-edg: expected an expression
// tidal-note: The second operand is missing in the initializer of a constexpr pointer.
constexpr int value = 3;
constexpr const int* p = &value ?: nullptr;
