// tidal-rules: REF-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct B { int v : 4; };
B b;
int& r = b.v;
