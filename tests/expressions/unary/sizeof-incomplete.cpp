// tidal-rules: UN-14
// tidal-expect: reject
// tidal-error-clang: sizeof. to an incomplete type
// tidal-error-gcc: sizeof. to incomplete type
// tidal-error-edg: incomplete type .Later. is not allowed
struct Later;
auto n = sizeof(Later);
