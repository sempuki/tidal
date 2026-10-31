// tidal-rules: DER-18
// tidal-expect: accept
struct A { int v; };
struct B : A {};
struct D : A, B {};
