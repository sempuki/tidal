// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: cannot cast from type .int \*. to member pointer type
// tidal-error-gcc: invalid .static_cast. from type .int\*. to type .int S::\*.
// tidal-error-edg: invalid type conversion
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
int x;
auto pm = static_cast<int S::*>(&x);
