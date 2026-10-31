// tidal-rules: MPTR-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { int v = 4; int get(this const S& self) { return self.v; } };
int main() { int (*p)(const S&) = &S::get; return p(S{}) == 4 ? 0 : 1; }
