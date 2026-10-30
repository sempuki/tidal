// tidal-rules: COND-5
// tidal-expect: reject
struct A {};
struct B {};
void f(bool c) { auto r = c ? A{} : B{}; }
