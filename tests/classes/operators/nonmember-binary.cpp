// tidal-rules: OPR-1
// tidal-expect: run
struct V { int x; };
V operator-(V a, V b) { return {a.x - b.x}; }
int main() { V a{5}, b{2}; return (a - b).x == 3 ? 0 : 1; }
