// tidal-rules: ACC-8
// tidal-expect: reject
struct B {};
class D : B {};
B* to_base(D* d) { return d; }
