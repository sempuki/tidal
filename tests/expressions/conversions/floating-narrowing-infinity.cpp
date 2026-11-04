// tidal-rules: CONV-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: does not fit in required floating-point type
// tidal-note: Q-0028. Clang and GCC give infinity for a double above the float range during constant evaluation. Only an infinity equals twice its value. EDG rejects the conversion.
constexpr float f = 1e300;
static_assert(f > 0 && f == f * 2.0f);
