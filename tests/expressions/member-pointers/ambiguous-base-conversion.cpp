// tidal-rules: MPTR-4
// tidal-expect: reject
struct A { int x; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int D::* pm = static_cast<int A::*>(&A::x);
