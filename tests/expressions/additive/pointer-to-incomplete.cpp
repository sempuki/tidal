// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: arithmetic on a pointer to an incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: pointer to a complete object type
struct Incomplete;
Incomplete* f(Incomplete* p) { return p + 1; }
