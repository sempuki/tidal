// tidal-rules: DER-5
// tidal-expect: run
struct B { int x = 1; };
struct D : B { int y = 2; };
int main() { D d; d.x = 7; B b = d; return b.x == 7 ? 0 : 1; }
