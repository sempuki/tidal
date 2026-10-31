// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: reinterpret_cast from .int S::\*. to .int \(S::\*\)\(\). is not allowed
// tidal-error-gcc: invalid cast from type .int S::\*.
// tidal-error-edg: invalid type conversion
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
auto pf = reinterpret_cast<int (S::*)()>(&S::a);
