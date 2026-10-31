// tidal-rules: DER-5
// tidal-expect: run
struct B { int x = 1; };
struct D : B { int y = 2; };
int main() { D d; B* p = &d; B& r = d; return p->x + r.x == 2 ? 0 : 1; }
