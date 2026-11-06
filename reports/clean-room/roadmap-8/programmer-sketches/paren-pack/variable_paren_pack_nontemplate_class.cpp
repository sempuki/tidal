struct P { int x, y; P(int a, int b) : x(a), y(b) {} };
template <class... A> int make(A... a) { P p(a...); return p.x + p.y; }
int main() { return make(1, 2) == 3 ? 0 : 1; }
