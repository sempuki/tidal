// tidal-rules: NAM-34
// tidal-expect: reject
// tidal-error-clang: qualified reference to .C. is a constructor name rather than a type
// tidal-error-gcc: type/value mismatch at argument 1
// tidal-error-edg: overloaded function .C::C. is not a type name
template <class T> struct box { };
struct C { };
box<C::C> b;
