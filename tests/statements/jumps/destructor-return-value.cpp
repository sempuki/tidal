// tidal-rules: JMP-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct S {
  ~S() { return 1; }
};
