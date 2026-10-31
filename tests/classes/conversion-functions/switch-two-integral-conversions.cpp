// tidal-rules: CVF-8
// tidal-expect: reject
struct S { operator int() const { return 1; } operator long() const { return 2; } };
int f(S s) { switch (s) { default: return 0; } }
