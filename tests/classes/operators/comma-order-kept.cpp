// tidal-rules: OPR-7
// tidal-expect: run
int trace = 0;
struct S { int v; };
S operator,(S a, S b) { return b; }
S mark(int d) { trace = trace * 10 + d; return {d}; }
int main() { S r = (mark(1), mark(2)); return trace == 12 ? 0 : 1; }
