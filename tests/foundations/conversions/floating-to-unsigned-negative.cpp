// tidal-rules: CONV-5
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression|\(unsigned int\)-1\.0e\+0. is not a constant
// tidal-error-edg: does not fit in required integral type
// tidal-error-clang: outside the range of representable values of type .const unsigned int
constexpr unsigned u = -1.0;
