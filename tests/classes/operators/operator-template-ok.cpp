// tidal-rules: OPR-1
// tidal-expect: accept
struct S { template <class T> S operator+(T) const { return *this; } };
S s;
S t = s + 1;
