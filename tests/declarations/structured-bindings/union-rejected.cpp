// tidal-rules: SB-3
// tidal-expect: reject
union U { int a; float b; };
U u;
auto [x] = u;
