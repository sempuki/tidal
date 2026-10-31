// tidal-rules: ILC-1
// tidal-expect: run
struct P { int x, y; P(int a, int b) : x(a), y(b * 10) {} };
int main() { P p{1, 2}; P q = {3, 4}; return p.y == 20 && q.x == 3 ? 0 : 1; }
