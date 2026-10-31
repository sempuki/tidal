// tidal-rules: STO-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { mutable int hits = 0; void touch() const { ++hits; } };
int main() { const S s; s.touch(); return s.hits == 1 ? 0 : 1; }
