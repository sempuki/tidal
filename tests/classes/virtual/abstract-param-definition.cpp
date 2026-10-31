// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: parameter type .A. is an abstract class
// tidal-error-gcc: cannot declare parameter .a. to be of abstract type
// tidal-error-edg: parameter of abstract class type
struct A { virtual void f() = 0; };
void take(A a) {}
