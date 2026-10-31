// tidal-rules: COND-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct B { int v = 1; };
struct D : B {};
int main() { D d; B b; B& r = (true ? d : b); r.v = 5; return d.v == 5 ? 0 : 1; }
