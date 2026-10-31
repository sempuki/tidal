// tidal-rules: DER-5
// tidal-expect: reject
struct B {};
struct D : B {};
D* f(B* p) { return p; }
