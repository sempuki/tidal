// tidal-rules: OPR-6
// tidal-expect: run
struct S { int v; int operator+(int) const { return 1; } };
int operator+(S, double) { return 2; }
int main() { S s{0}; return (s + 1) == 1 && (s + 1.0) == 2 ? 0 : 1; }
