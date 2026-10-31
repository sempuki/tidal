// tidal-rules: UN-14
// tidal-expect: reject
// tidal-error-clang: sizeof. to an incomplete type .void
// tidal-error-gcc: sizeof. to a void type
// tidal-error-edg: incomplete type .void. is not allowed
auto n = sizeof(void);
