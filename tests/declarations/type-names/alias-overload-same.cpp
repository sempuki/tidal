// tidal-rules: TYN-8
// tidal-expect: reject
// tidal-error-clang: redefinition of .f
// tidal-error-gcc: redefinition of .void f\(int\)
// tidal-error-edg: has already been defined
using Int = int;
void f(int);
void f(Int) {}
void f(int) {}
