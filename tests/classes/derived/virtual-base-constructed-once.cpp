// tidal-rules: DER-7
// tidal-expect: run
int count = 0;
struct A { A() { ++count; } };
struct B : virtual A {};
struct C : virtual A {};
struct D : B, C {};
int main() { D d; return count == 1 ? 0 : 1; }
