// tidal-rules: OPR-4
// tidal-expect: reject
struct C { C& operator++(); };
void f(C c) { c++; }
