// tidal-rules: COND-5
// tidal-expect: reject
struct B;
struct A { A(); A(const B&); };
struct B { B(); B(const A&); };
void f(bool c) { auto r = c ? A{} : B{}; }
