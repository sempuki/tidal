// tidal-rules: UNEV-1
// tidal-expect: reject
// tidal-error-clang: call to deleted function
// tidal-error-gcc: use of deleted function .int f\(\).
// tidal-error-edg: it is a deleted function
int f() = delete;
using T = decltype(f());
