// tidal-rules: DER-11
// tidal-expect: reject
// tidal-error-clang: via virtual base
// tidal-error-gcc: because the base is virtual
// tidal-error-edg: base class is virtual
// tidal-note: A reference cast to a class two levels down, where the virtual base is a base of the middle class.
struct A {};
struct B : virtual A {};
struct C : B {};
C& f(A& a) { return static_cast<C&>(a); }
