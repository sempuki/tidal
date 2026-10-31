// tidal-rules: OPR-1
// tidal-expect: run
struct V { int x;  V operator+(V o) const { return {x + o.x}; } };
int main() { V a{1}, b{2}; return (a + b).x == 3 ? 0 : 1; }
