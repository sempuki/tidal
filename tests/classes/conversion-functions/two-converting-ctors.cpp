// tidal-rules: CVF-2
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .take
// tidal-error-gcc: could not convert .3. from .int. to .D
// tidal-error-edg: no suitable constructor exists to convert from .int. to .D
struct A { A(int) {} };
struct D { D(A) {} };
void take(D);
void f() { take(3); }
