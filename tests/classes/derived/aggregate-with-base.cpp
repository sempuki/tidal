// tidal-rules: DER-2
// tidal-expect: run
struct B { int x; };
struct D : B { int y; };
int main() { D d{{1}, 2}; D e{3, 4}; return d.x + d.y + e.x + e.y == 10 ? 0 : 1; }
