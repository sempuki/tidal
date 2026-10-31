// tidal-rules: CONV-17
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from pointer to member
// tidal-error-gcc: is an ambiguous base of
// tidal-error-edg: contains more than one instance of class
// tidal-note: Gap review 2026-10-31 (third).
struct A { int x; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int A::* pa = &A::x;
int D::* pm = pa;
