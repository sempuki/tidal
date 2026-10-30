// tidal-rules: CPY-9
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };
int main() { const S a; S b(static_cast<const S&&>(a)); return copies == 1 && moves == 0 ? 0 : 1; }
