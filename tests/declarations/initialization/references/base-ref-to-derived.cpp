// tidal-rules: REF-1
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct B { int v = 0; };
struct D : B { int w = 0; };
int main() { D d; d.v = 7; B& b = d; return b.v == 7 ? 0 : 1; }
