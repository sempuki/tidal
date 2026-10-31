// tidal-rules: NAM-6
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .n
// tidal-error-gcc: n. was not declared in this scope
// tidal-error-edg: identifier .n. is undefined
struct S { static int n; };
int n_copy = n;
