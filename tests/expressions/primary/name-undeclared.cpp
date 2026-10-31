// tidal-rules: PRIM-12 NAM-8
// tidal-expect: reject
// tidal-error-clang: use of undeclared identifier
// tidal-error-gcc: was not declared in this scope
// tidal-error-edg: is undefined
int f() { return undeclared_name; }
