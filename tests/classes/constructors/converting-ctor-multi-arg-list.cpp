// tidal-rules: CTOR-10
// tidal-expect: run
struct P { int x, y; P(int a, int b) : x(a), y(b) {} };
int sum(P p) { return p.x + p.y; }
int main() { return sum({2, 3}) == 5 ? 0 : 1; }
