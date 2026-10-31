// tidal-rules: THR-3
// tidal-expect: reject
// tidal-error-clang: cannot throw an object of abstract type
// tidal-error-gcc: abstract class type .A. cannot be used in throw-expression
// tidal-error-edg: object of abstract class type .A. is not allowed
struct A { virtual void f() = 0; };
void g(A& a) { throw a; }
