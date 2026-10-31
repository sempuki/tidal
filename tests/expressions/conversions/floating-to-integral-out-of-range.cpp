// tidal-rules: CONV-5
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression|\(int\)1\.0e\+10. is not a constant
// tidal-error-edg: does not fit in required integral type
constexpr int big = 1e10;
