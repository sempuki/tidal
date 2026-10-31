// tidal-rules: VIRT-8
// tidal-expect: accept
struct A { virtual void f() = 0; };
void g(A& a) { auto l = [&a] { a.f(); }; l(); }
