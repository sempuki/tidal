// tidal-rules: VC-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct S { int v = 7; ~S() { mark(1); } };
int main() { int S::* pm = &S::v; int x = S{}.*pm; mark(2); return x == 7 && trace == 12 ? 0 : 1; }
