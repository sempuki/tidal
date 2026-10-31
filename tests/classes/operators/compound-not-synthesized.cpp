// tidal-rules: OPR-6
// tidal-expect: reject
struct V { int x;  V operator+(V o) const { return {x + o.x}; } };
void f(V& a, V b) { a += b; }
