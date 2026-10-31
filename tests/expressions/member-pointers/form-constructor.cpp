// tidal-rules: MPTR-2
// tidal-expect: reject
struct C { C(); };
auto p = &C::C;
