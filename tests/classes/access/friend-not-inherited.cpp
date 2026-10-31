// tidal-rules: ACC-6
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
class C { int secret; friend class A; };
class A {};
class B : public A { int look(const C& c) { return c.secret; } };
