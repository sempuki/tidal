// tidal-rules: CTOR-10
// tidal-expect: accept
struct A { A(long) {} };
void take(A);
void f() { take('x'); }
