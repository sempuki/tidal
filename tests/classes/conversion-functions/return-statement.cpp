// tidal-rules: CVF-2
// tidal-expect: run
struct S { operator int() const { return 8; } };
int get() { return S{}; }
int main() { return get() == 8 ? 0 : 1; }
