// tidal-rules: DER-8 CTOR-2
// tidal-expect: accept
// tidal-note: B has only a default and a copy constructor, so D inherits no constructor. D keeps its implicit default, copy, and move constructors.
struct B { B() {} B(const B&) {} };
struct D : B { using B::B; };
D first;
D second(first);
D third(static_cast<D&&>(first));
