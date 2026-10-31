// tidal-rules: INIT-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int trace = 0;
int mark(int d) { trace = trace * 10 + d; return d; }
int main() { int a = mark(1), b = mark(2); return trace == 12 && a + b == 3 ? 0 : 1; }
