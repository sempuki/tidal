// tidal-rules: UNEV-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct D { ~D() = delete; };
D make();
auto n = sizeof(make());
