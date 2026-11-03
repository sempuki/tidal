// tidal-rules: DER-18
// tidal-expect: accept
// tidal-note: The direct base comes after the path that holds it indirectly, two levels deep.
struct A { int v; };
struct B : A {};
struct C : B {};
struct D : C, A {};
D d;
