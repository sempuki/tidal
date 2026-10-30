// tidal-rules: CTOR-10
// tidal-expect: run
struct Meters { int v; Meters(int x) : v(x) {} };
int twice(Meters m) { return m.v * 2; }
int main() { return twice(4) == 8 ? 0 : 1; }
