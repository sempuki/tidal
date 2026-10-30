// tidal-rules: CPY-12
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };
S make() { S s; return s; }
int main() { S a = make(); return copies == 0 && moves <= 1 ? 0 : 1; }
