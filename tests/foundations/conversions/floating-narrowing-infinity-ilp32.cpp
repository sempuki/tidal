// tidal-rules: CONV-11
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg: reject
// tidal-error-edg: does not fit in required floating-point type
// tidal-note: Q-0028. -m32. The same choices as on x86-64 (floating-narrowing-infinity.cpp).
constexpr float f = 1e300;
static_assert(f > 0 && f == f * 2.0f);
