// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct S {
  int v;
  int get(this S self) { return this->v; }
};
