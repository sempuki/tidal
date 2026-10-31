// tidal-rules: OPR-6
// tidal-expect: reject
struct S { S(int) {} S operator+(S) const { return *this; } };
S f() { return 1 + S(2); }
