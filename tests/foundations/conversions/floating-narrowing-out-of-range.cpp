// tidal-rules: CONV-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: does not fit in required floating-point type
// tidal-note: Q-0028. Clang and GCC round a double above the float range to infinity during constant evaluation; EDG rejects it. CONV-11 makes the choice implementation-defined, so it allows both answers. The base line gives the choice of Clang and GCC.
constexpr float f = 1e300;
static_assert(f > 3.0e38f);
