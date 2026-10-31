// tidal-rules: MPTR-11
// tidal-expect: reject
struct A { int x; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int D::* pm = static_cast<int A::*>(&A::x);
