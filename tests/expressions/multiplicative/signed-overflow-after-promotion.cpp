// tidal-rules: MUL-10
// tidal-expect: reject
// tidal-error-clang: outside the range of representable values of type .int
// tidal-error-edg: value exceeds range of .int
constexpr unsigned short big = 65535;
static_assert(sizeof(int) == 4);
static_assert(big * big != 0);
