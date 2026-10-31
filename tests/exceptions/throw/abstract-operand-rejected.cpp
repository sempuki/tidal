// tidal-rules: THR-3
// tidal-expect: reject
struct A { virtual void f() = 0; };
void g(A& a) { throw a; }
