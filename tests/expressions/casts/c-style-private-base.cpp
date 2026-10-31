// tidal-rules: CAST-17
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct B { int b = 4; };
struct D : private B {};
int main() { D d; B* p = (B*)&d; return p->b == 4 ? 0 : 1; }
