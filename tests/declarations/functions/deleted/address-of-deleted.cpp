// tidal-rules: DEL-1
// tidal-expect: reject
// tidal-error-clang: attempt to use a deleted function
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
void f() = delete;
auto p = &f;
