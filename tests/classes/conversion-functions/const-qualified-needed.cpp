// tidal-rules: CVF-1
// tidal-expect: reject
struct S { operator int() { return 1; } };
int f(const S& s) { return s; }
