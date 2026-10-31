// tidal-rules: CVF-3
// tidal-expect: reject
struct S { explicit operator int() const { return 1; } };
int f() { return S{} + 1; }
