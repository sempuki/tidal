// tidal-rules: DER-4
// tidal-expect: accept
struct A { static int count; enum { kind = 3 }; using type = int; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int f(D& d) { D::type t = d.kind; return t + d.count; }
