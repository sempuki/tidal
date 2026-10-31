// tidal-rules: DER-7
// tidal-expect: run
struct A { int v; A(int x) : v(x) {} };
struct B : virtual A { B() : A(1) {} };
struct C : virtual A { C() : A(2) {} };
struct D : B, C { D() : A(3) {} };
int main() { D d; return d.v == 3 ? 0 : 1; }
