// tidal-rules: CPY-5
// tidal-expect: run
int copies = 0;
int moves = 0;

struct M { M() {} M(const M&) { ++copies; } M(M&&) { ++moves; } };
struct S { M m; S() {} S& operator=(const S&) { return *this; } };
int main() { S a; S b(static_cast<S&&>(a)); return copies == 1 && moves == 0 ? 0 : 1; }
