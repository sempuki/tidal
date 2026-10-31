// tidal-rules: CVF-2
// tidal-expect: run
struct S { operator int() const { return 4; } };
int main() { double d = S{}; long l = S{}; return d == 4.0 && l == 4 ? 0 : 1; }
