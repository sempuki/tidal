// tidal-rules: VOL-2
// tidal-expect: run
struct S { int v; S() : v(2) { v = 3; } };
volatile S s;
int main() { return s.v == 3 ? 0 : 1; }
