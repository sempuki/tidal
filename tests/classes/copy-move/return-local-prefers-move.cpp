// tidal-rules: CPY-11
// tidal-expect: run
int copies = 0;
int moves = 0;

struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };
S make(bool b) { S x, y; if (b) return x; return y; }
int main() { S s = make(true); return copies == 0 ? 0 : 1; }
