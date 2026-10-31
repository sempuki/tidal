// tidal-rules: DEL-1
// tidal-expect: reject
// tidal-error-clang: call to deleted function
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
void f() = delete;
void g() { f(); }
