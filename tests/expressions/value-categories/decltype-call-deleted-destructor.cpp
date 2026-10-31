// tidal-rules: VC-5
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct D { ~D() = delete; };
D make();
using T = decltype(make());
