// tidal-rules: ACC-17
// tidal-expect: reject
// tidal-error-clang: undeclared identifier
// tidal-error-gcc: was not declared in this scope
// tidal-error-edg: is undefined
class Point {
  friend int make_zero() { return 0; }
};
int z = make_zero();
