// tidal-rules: DTOR-8
// tidal-expect: reject
// tidal-error-clang: does not match the type .Derived. of the object being destroyed
// tidal-expect-gcc: accept
// tidal-error-edg: invalid destructor name
// tidal-note: Read of the RV7 repair diff, RV8-R-1, and ledger row RM12-3. In d.~Base(), the name after ~ is the base class and the object is a Derived, so the name does not denote the type of the object. Clang and EDG reject the call. GCC accepts it and runs the base destructor, as the review reads the reference (Q-0444). The qualified form d.Base::~Base() is valid on every compiler (dtor-call-qualified-base.cpp).
struct Base { ~Base() {} };
struct Derived : Base { int count = 0; };
void end_base(Derived& item) { item.~Base(); }
