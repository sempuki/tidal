// tidal-rules: MPTR-9
// tidal-expect: reject
struct C { C(); };
auto p = &C::C;
