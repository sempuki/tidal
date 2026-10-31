// tidal-rules: NAM-34
// tidal-expect: reject
// tidal-error-clang: qualified reference to .point. is a constructor name rather than a type
// tidal-error-gcc: applying .sizeof. to an expression of function type
// tidal-error-edg: cannot determine which instance of overloaded function .point::point. is intended
struct point { int x = 1; };
static_assert(sizeof(point::point) == sizeof(point));
