// tidal-rules: STMT-10
// tidal-expect: reject
// tidal-error-clang: must use .struct. tag to refer to type .Point
// tidal-error-gcc: expected .;. before .p
// tidal-error-edg: expected a .;.
int f() {
  struct Point { int x; };
  int Point = 3;
  Point p{4};
  return Point + p.x;
}
