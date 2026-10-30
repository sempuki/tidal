// tidal-rules: DEFARG-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct S {
  int f(const S* p = this);
};
