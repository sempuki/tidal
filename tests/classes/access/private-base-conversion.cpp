// tidal-rules: ACC-12
// tidal-expect: reject
// tidal-error-clang: private base class
// tidal-error-gcc: inaccessible base
// tidal-error-edg: inaccessible base class
struct B {};
class D : B {};
B* to_base(D* d) { return d; }
