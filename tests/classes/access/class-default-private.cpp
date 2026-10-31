// tidal-rules: ACC-1
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
class C { int v; };
int f(C c) { return c.v; }
