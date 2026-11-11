struct P { int x, y; P(int a, int b) : x(a), y(b) {} };
template <class T, class... A> int make(A... a) { T* p = new T(a...); int r = p->x + p->y; delete p; return r; }
int main() { return make<P>(1, 2) == 3 ? 0 : 1; }
