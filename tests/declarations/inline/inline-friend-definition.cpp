// tidal-rules: INL-3
// tidal-expect: run
struct S { friend int peek(S) { return 4; } };
int main() { return peek(S{}) == 4 ? 0 : 1; }
