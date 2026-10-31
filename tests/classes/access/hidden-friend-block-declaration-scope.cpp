// tidal-rules: ACC-17
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .make_seven.
// tidal-error-gcc: make_seven. was not declared in this scope
// tidal-error-edg: identifier .make_seven. is undefined
class Point {
  friend int make_seven() { return 7; }
};
void declare() { int make_seven(void); }
int seven = make_seven();
