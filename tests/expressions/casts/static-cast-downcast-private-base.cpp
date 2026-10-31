// tidal-rules: CAST-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct B {};
struct D : private B {};
D* f(B* p) { return static_cast<D*>(p); }
