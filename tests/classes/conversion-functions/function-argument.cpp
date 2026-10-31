// tidal-rules: CVF-2
// tidal-expect: run
struct S { operator int() const { return 4; } };
int twice(int x) { return x * 2; }
int main() { return twice(S{}) == 8 ? 0 : 1; }
