// tidal-rules: OVL-6 OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching function for call to object of type .Temp.
// tidal-error-gcc: passing .Temp. as .this. argument discards qualifiers
// tidal-error-edg: call of an object of a class type without appropriate operator\(\) or conversion functions
// tidal-note: The conversion function is qualified &&, so an lvalue Temp cannot convert by it.
using Fn = int (*)(int);
int twice(int x) { return 2 * x; }
struct Temp { operator Fn() && { return twice; } };
int use(Temp t) { return t(3); }
