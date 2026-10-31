// tidal-rules: MPTR-1
// tidal-expect: reject
struct S { int& r; };
auto pm = &S::r;
