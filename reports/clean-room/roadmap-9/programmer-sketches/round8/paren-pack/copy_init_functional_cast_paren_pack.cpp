struct P { int x, y; P(int a, int b) : x(a), y(b) {} };
template <class T, class... A> int make(A... a) { T x = T(a...); return x.x + x.y; }
int main() { return make<P>(1, 2) == 3 ? 0 : 1; }
