// tidal-rules: TYP-4
// tidal-expect: reject
// tidal-error-clang: outside the range of representable values of type .int.
// tidal-error-gcc: overflow in constant expression
// tidal-error-edg: value exceeds range of .int.
constexpr int int_max = 2147483647;
constexpr int past = int_max + 1;
