template <class T> T&& fwd(T& t) { return static_cast<T&&>(t); }
struct P { int x, y; P(int a, int b) : x(a), y(b) {} };
template <class T> struct W { T v; template <class... A> W(A&&... a) : v(fwd<A>(a)...) {} };
int main() { W<P> w(1, 2); return w.v.x + w.v.y == 3 ? 0 : 1; }
