// tidal-rules: ACC-8
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
struct B { int x; };
class D : B {};
int f(D& d) { return d.x; }
