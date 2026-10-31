// tidal-rules: NAM-36 ACC-14
// tidal-expect: reject
// tidal-error-clang: A. is a private member of .A.
// tidal-error-gcc: A A::A. is private within this context
// tidal-error-edg: type .A::A. .declared at line [0-9]+. is inaccessible
struct A { };
struct B : private A { };
struct D : B { void f() { A* p = nullptr; (void)p; } };
