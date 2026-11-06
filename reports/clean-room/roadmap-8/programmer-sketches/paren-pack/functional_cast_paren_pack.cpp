struct P { int x, y; P(int a, int b) : x(a), y(b) {} };
template <class T, class... A> int make(A... a) { return T(a...).x + T(a...).y; }
int main() { return make<P>(1, 2) == 3 ? 0 : 1; }
