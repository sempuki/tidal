// tidal-rules: FN-16 NAM-8
// tidal-expect: reject
// tidal-error-clang: use of undeclared identifier .count.
// tidal-error-gcc: .count. was not declared in this scope
// tidal-error-edg: identifier "count" is undefined
int twice(int count);
int twice(int amount) { return count * 2; }
