// tidal-rules: DTOR-8
// tidal-expect: reject
// tidal-error-clang: does not match the type being destroyed
// tidal-error-gcc: is not of type .L
// tidal-error-edg: invalid destructor name for type .int
using L = long;
void f(int i) { i.~L(); }
