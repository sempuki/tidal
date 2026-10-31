// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: field type .A. is an abstract class
// tidal-error-gcc: cannot declare field .S::a. to be of abstract type
// tidal-error-edg: object of abstract class type
struct A { virtual void f() = 0; };
struct S { A a; };
