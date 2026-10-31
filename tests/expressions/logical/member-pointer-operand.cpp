// tidal-rules: LOG-12
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct S { int m; };
int main() { int S::* p = nullptr; int S::* q = &S::m; return (!p && q) ? 0 : 1; }
