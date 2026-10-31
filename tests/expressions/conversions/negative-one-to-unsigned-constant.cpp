// tidal-rules: CONV-5
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression|\(unsigned int\)-1\.0e\+0. is not a constant
// tidal-error-edg: does not fit in required integral type
// tidal-note: Gap review 2026-10-31 (third).
constexpr unsigned u = -1.0;
