// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: allocating an object of abstract class type
// tidal-error-gcc: invalid new-expression of abstract class type
// tidal-error-edg: object of abstract class type
struct A { virtual void f() = 0; };
A* p = new A;
