// tidal-rules: CONV-5
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression|\(unsigned int\)-1\.0e\+0. is not a constant
// tidal-error-edg: does not fit in required integral type
constexpr unsigned u = -1.0;
