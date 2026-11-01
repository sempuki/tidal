// tidal-rules: MUL-10
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: overflow in constant expression
// tidal-error-clang: outside the range of representable values of type .long long
// tidal-note: Q-0003. EDG 7.0 accepts the most negative long long divided by -1 during constant evaluation.
constexpr long long lowest = -(long long)(~0ull >> 1) - 1;
constexpr long long q = lowest / -1;
