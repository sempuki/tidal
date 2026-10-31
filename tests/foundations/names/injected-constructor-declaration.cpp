// tidal-rules: NAM-34
// tidal-expect: reject
// tidal-error-clang: qualified reference to .box. is a constructor name rather than a type
// tidal-error-gcc: box::box. names the constructor, not the type
// tidal-error-edg: overloaded function .box::box. is not a type name
struct box { };
box::box b;
