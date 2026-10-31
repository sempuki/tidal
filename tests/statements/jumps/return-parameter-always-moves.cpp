// tidal-rules: JMP-11
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int copies = 0, moves = 0;
struct C { C() {} C(const C&) { ++copies; } C(C&&) { ++moves; } };
C pass(C c) { return c; }
int main() { C a; copies = 0; C b = pass(static_cast<C&&>(a)); moves = 0; copies = 0;
  C d = pass(C{}); return copies == 0 && moves == 1 ? 0 : 1; }
