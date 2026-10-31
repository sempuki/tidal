// tidal-rules: DER-6
// tidal-expect: run
struct A { int v = 0; };
struct B : virtual A {};
struct C : virtual A {};
struct D : B, C {};
int main() { D d; static_cast<B&>(d).v = 5; return static_cast<C&>(d).v == 5 && d.v == 5 ? 0 : 1; }
