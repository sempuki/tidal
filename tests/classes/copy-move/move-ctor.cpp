// tidal-rules: CPY-2
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };
int main() { S a; S b(static_cast<S&&>(a)); return moves == 1 && copies == 0 ? 0 : 1; }
