// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from pointer to member
// tidal-error-gcc: is an ambiguous base of
// tidal-error-edg: contains more than one instance of class
struct A { int x; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int D::* pm = static_cast<int A::*>(&A::x);
