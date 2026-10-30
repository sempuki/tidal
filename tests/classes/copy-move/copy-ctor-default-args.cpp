// tidal-rules: CPY-1
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&, int extra = 0) { copies += 1 + extra; } };
int main() { S a; S b(a); return copies == 1 ? 0 : 1; }
