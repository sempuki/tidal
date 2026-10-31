// tidal-rules: OPR-5
// tidal-expect: reject
struct A { int x; A& operator->(); };
int f(A& a) { return a->x; }
