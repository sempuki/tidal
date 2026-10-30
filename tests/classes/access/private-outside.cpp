// tidal-rules: ACC-2
// tidal-expect: reject
class C { int secret = 1; public: C() {} };
int f(const C& c) { return c.secret; }
