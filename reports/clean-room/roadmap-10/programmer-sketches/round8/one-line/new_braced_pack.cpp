struct P { int x, y; };
template <class T, class... A> T* make(A... a) { return new T{a...}; }
int main() { P* p = make<P>(1, 2); int r = p->x + p->y; delete p; return r == 3 ? 0 : 1; }
