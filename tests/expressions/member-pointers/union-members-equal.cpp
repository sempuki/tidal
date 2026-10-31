// tidal-rules: MPTR-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
union U { int a; int b; };
int main() { int U::* p = &U::a; int U::* q = &U::b; return p == q ? 0 : 1; }
