// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: variable type .* is an abstract class
// tidal-error-gcc: cannot declare variable .* to be of abstract type
// tidal-error-edg: object of abstract class type
struct A { virtual void f() = 0; };
A a;
