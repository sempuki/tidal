// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: allocating an object of abstract class type
// tidal-error-gcc: invalid cast to abstract class type
// tidal-error-edg: cast to abstract class .A. is not allowed
struct A { virtual void f() = 0; };
void g() { A(); }
