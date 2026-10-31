// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: variable type .* is an abstract class
// tidal-error-gcc: cannot declare variable .* to be of abstract type
// tidal-error-edg: object of abstract class type
struct A { virtual void f() {} };
struct B : A { void f() override = 0; };
B b;
