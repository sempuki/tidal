// tidal-rules: OPR-6
// tidal-expect: run
struct S { operator int() const { return 2; } };
int main() { S a, b; return a * b == 4 ? 0 : 1; }
