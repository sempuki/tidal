// tidal-rules: CVF-4
// tidal-expect: run
struct S { operator int() const { return 1; } operator long() const { return 2; } };
int main() { long l = S{}; int i = S{}; return l == 2 && i == 1 ? 0 : 1; }
