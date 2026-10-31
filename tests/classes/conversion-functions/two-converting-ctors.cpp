// tidal-rules: CVF-2
// tidal-expect: reject
struct A { A(int) {} };
struct D { D(A) {} };
void take(D);
void f() { take(3); }
