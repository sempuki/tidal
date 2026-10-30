// tidal-rules: JMP-10
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct M {
  M(int) {}
  M(M&&) {}
  M(const M&) = delete;
};
M pass(M m) { return m; }
