// tidal-rules: CONV-18
// tidal-expect: reject
// tidal-error-clang: type .int S::\*
// tidal-error-gcc: cannot convert .int S::\*. to .long
// tidal-error-edg: type "int S::\*" cannot be used to initialize
struct S { int m; };
int S::* pm = &S::m;
long n = pm;
