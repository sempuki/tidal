// tidal-rules: CTOR-10
// tidal-expect: reject
struct A { A(int) {} };
struct B { B(A) {} };
void take(B);
void f() { take(1); }
