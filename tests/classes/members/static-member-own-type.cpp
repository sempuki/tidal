// tidal-rules: CLS-16
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { int v = 3; static S self; };
S S::self;
int main() { return S::self.v == 3 ? 0 : 1; }
