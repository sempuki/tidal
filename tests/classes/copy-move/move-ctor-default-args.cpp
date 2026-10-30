// tidal-rules: CPY-2
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(S&&, int = 0) { ++moves; } };
int main() { S a; S b(static_cast<S&&>(a)); return moves == 1 ? 0 : 1; }
