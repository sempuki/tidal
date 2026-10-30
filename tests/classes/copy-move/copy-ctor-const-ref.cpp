// tidal-rules: CPY-1
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&) { ++copies; } };
int main() { S a; S b(a); S c = a; return copies == 2 ? 0 : 1; }
