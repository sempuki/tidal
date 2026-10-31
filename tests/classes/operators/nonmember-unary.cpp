// tidal-rules: OPR-1
// tidal-expect: run
struct V { int x; };
bool operator!(V v) { return v.x == 0; }
int main() { V a{0}; return !a ? 0 : 1; }
