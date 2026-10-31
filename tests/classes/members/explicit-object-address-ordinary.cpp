// tidal-rules: CLS-17
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { int v; int get(this const S& s) { return s.v; } };
int main() { int (*p)(const S&) = &S::get; S s{7}; return p(s) == 7 ? 0 : 1; }
