// tidal-rules: NAM-34
// tidal-expect: reject
// tidal-error-clang: qualified reference to .point. is a constructor name rather than a type
// tidal-error-gcc: cannot call constructor .point::point. directly
// tidal-error-edg: a fully qualified constructor call is not allowed
struct point { int x = 1; };
int read() { return point::point().x; }
