// tidal-rules: CPY-11
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int copies = 0, moves = 0;
struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };
S pass(S p) { return p; }
int main() { S a = pass(S{}); return copies == 0 && moves == 1 ? 0 : 1; }
