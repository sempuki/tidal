// tidal-rules: CPY-9
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };
S make() { S s; return static_cast<S&&>(s); }
int main() { S a; S b(a); S c(static_cast<S&&>(a)); return copies == 1 && moves >= 1 ? 0 : 1; }
