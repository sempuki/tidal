// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: capture of value of abstract type
// tidal-error-gcc: abstract type .A
// tidal-error-edg: object of abstract class type
struct A { virtual void f() = 0; };
void g(A& a) { auto l = [a] { }; }
