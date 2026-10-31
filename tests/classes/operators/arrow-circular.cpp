// tidal-rules: OPR-5
// tidal-expect: reject
// tidal-error-clang: circular pointer delegation
// tidal-error-gcc: circular pointer delegation
// tidal-error-edg: loop in sequence of .operator->
struct A { int x; A& operator->(); };
int f(A& a) { return a->x; }
