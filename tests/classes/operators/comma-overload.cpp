// tidal-rules: OPR-1
// tidal-expect: run
struct S { int v; };
S operator,(S a, S b) { return {a.v * 10 + b.v}; }
int main() { S a{1}, b{2}; S c = (a, b); return c.v == 12 ? 0 : 1; }
