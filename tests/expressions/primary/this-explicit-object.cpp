// tidal-rules: PRIM-20
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { int v; int get(this S& self) { return this->v; } };
