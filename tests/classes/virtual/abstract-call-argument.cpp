// tidal-rules: VIRT-8
// tidal-expect: reject
// tidal-error-clang: abstract class type
// tidal-error-gcc: object of abstract type
// tidal-error-edg: abstract class type
// tidal-note: The declaration of take is valid. The call makes its argument as an object of the abstract class A, so the call is rejected.
struct A { virtual void f() = 0; };
void take(A);
void g(A& a) { take(a); }
