// tidal-rules: CPY-7
// tidal-expect: run
int copies = 0;
int moves = 0;

struct M { M() {} M(const M&) { ++copies; } M(M&&) = delete; };
struct S { M m; S() {} S(const S&) = default; S(S&&) = default; };
int main() { S a; S b(static_cast<S&&>(a)); return copies == 1 ? 0 : 1; }
