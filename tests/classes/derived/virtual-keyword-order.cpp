// tidal-rules: DER-6
// tidal-expect: accept
struct A {};
struct B : public virtual A {};
struct C : virtual public A {};
