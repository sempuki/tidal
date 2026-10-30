// tidal-rules: CPY-14
// tidal-expect: run
struct S { int v = 3; S() {} S(const S&); };
S::S(const S&) = default;
int main() { S a; a.v = 7; S b(a); return b.v == 7 ? 0 : 1; }
