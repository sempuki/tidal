// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: return type .A. is an abstract class
// tidal-error-gcc: invalid abstract return type
// tidal-error-edg: function returning abstract class
struct A { virtual void f() = 0; };
A make() { while (true) {} }
