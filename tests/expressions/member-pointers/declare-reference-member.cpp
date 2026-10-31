// tidal-rules: MPTR-8
// tidal-expect: reject
struct S { int& r; };
auto pm = &S::r;
