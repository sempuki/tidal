// tidal-rules: ACC-6
// tidal-expect: reject
class C { int secret; friend class A; };
class A {};
class B : public A { int look(const C& c) { return c.secret; } };
