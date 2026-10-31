// tidal-rules: STO-12
// tidal-expect: reject
// tidal-error-clang: non-thread-local declaration of .u. follows thread-local declaration
// tidal-error-gcc: non-thread-local declaration of .int u. follows thread-local declaration
// tidal-error-edg: non-thread-local declaration follows thread-local declaration
thread_local int u;
extern int u;
