// tidal-rules: MPTR-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { int m; };
int S::* zeroed;
int main() { int S::* a = 0; return a == nullptr && zeroed == nullptr ? 0 : 1; }
