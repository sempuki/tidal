// tidal-rules: CVF-2
// tidal-expect: run
struct S { operator int() const { return 2; } };
int main() { switch (S{}) { case 2: return 0; default: return 1; } }
