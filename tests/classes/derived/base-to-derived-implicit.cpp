// tidal-rules: DER-10
// tidal-expect: reject
struct B {};
struct D : B {};
D* f(B* p) { return p; }
