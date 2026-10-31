// tidal-rules: OPR-6
// tidal-expect: run
struct S { int v; S(int x) : v(x) {} };
S operator+(S a, S b) { return S(a.v + b.v); }
int main() { S s = 1 + S(2); return s.v == 3 ? 0 : 1; }
