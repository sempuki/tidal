// tidal-rules: UNEV-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct D { ~D() = delete; };
D make();
using T = decltype(make());
using U = decltype(0, make());
