// tidal-rules: CVF-2
// tidal-expect: run
struct S { operator int() const { return 4; } };
int main() { S s; return s + 1 == 5 && s * s == 16 ? 0 : 1; }
