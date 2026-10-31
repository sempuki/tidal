// tidal-rules: NAM-34
// tidal-expect: reject
// tidal-error-clang: qualified reference to .C. is a constructor name rather than a type
// tidal-error-gcc: C::C. names the constructor, not the type
// tidal-error-edg: overloaded function .C::C. is not a type name
struct C { };
using alias = C::C;
