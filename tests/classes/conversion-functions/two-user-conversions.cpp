// tidal-rules: CVF-10
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .take
// tidal-error-gcc: could not convert .A\(\). from .A. to .B
// tidal-error-edg: no suitable user-defined conversion from .A. to .B
struct A { operator int() const { return 1; } };
struct B { B(int) {} };
void take(B);
void f() { take(A{}); }
