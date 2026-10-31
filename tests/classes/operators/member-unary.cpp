// tidal-rules: OPR-1
// tidal-expect: run
struct V { int x;  V operator-() const { return {-x}; } };
int main() { V a{4}; return (-a).x == -4 ? 0 : 1; }
