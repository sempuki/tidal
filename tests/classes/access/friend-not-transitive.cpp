// tidal-rules: ACC-6
// tidal-expect: reject
class C { int secret; friend class A; };
class A { friend class B; };
class B { int look(const C& c) { return c.secret; } };
