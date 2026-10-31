// tidal-rules: PRIM-11
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { int v; auto get() const -> decltype(this->v) { return v; } };
