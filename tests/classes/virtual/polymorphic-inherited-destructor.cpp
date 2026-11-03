// tidal-rules: VIRT-11
// tidal-expect: accept
// tidal-note: B declares no virtual function and inherits a virtual destructor, so a pointer to B can be the operand of dynamic_cast.
struct A { virtual ~A(); };
struct B : A {};
struct C : B {};
C* to_c(B* b) { return dynamic_cast<C*>(b); }
void* whole(B* b) { return dynamic_cast<void*>(b); }
