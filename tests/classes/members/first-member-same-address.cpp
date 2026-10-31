// tidal-rules: CLS-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { int a; int b; };
int main() { S s; return static_cast<void*>(&s) == static_cast<void*>(&s.a) ? 0 : 1; }
