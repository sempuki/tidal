// tidal-rules: OPR-1
// tidal-expect: run
struct V { int x;  V operator+(V o) const { return {x + o.x}; } };
V operator*(V a, V b) { return {a.x * b.x}; }
int main() { V a{2}, b{3}; return a.operator+(b).x == 5 && operator*(a, b).x == 6 ? 0 : 1; }
