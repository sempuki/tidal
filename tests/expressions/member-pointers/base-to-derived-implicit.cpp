// tidal-rules: MPTR-4
// tidal-expect: run
struct B { int x = 4; };
struct D : B { int y = 0; };
int main() { int D::* pm = &B::x; D d; return d.*pm == 4 ? 0 : 1; }
