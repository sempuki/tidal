// tidal-rules: CVF-1
// tidal-expect: run
struct S { operator int() const { return 3; } };
int main() { S s; return s.operator int() == 3 ? 0 : 1; }
