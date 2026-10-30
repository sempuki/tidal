// tidal-rules: CPY-10
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };
S make() { return S(); }
int main() { S a = make(); S b = S(S()); return copies + moves == 0 ? 0 : 1; }
