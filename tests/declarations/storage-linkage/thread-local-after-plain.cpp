// tidal-rules: STO-12
// tidal-expect: reject
// tidal-error-clang: thread-local declaration of .t. follows non-thread-local declaration
// tidal-error-gcc: thread-local declaration of .int t. follows non-thread-local declaration
// tidal-error-edg: thread-local declaration follows non-thread-local declaration
extern int t;
thread_local int t;
