// tidal-rules: NAM-1
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .later
// tidal-error-gcc: later. was not declared in this scope
// tidal-error-edg: identifier .later. is undefined
int get() { return later; }
int later = 5;
