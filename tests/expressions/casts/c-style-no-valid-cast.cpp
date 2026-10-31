// tidal-rules: CAST-19
// tidal-expect: reject
// tidal-error-clang: cannot convert .Point. to .int.
// tidal-error-gcc: invalid cast from type
// tidal-error-edg: no suitable conversion function
struct Point { int x; int y; };
int flat = (int)Point{1, 2};
