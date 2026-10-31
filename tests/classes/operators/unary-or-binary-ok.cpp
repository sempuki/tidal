// tidal-rules: OPR-2
// tidal-expect: accept
struct S { S operator-() const; S operator-(S) const; S operator*() const; S operator*(S) const; };
