// tidal-rules: CVF-4
// tidal-expect: reject
struct S { operator int() const { return 1; } operator long() const { return 2; } };
double d = S{};
