// CTAD-1: a template template parameter and a dependent qualifier stand without arguments.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Box { T v; constexpr Box(T x) : v(x) {} };
template <template <class> class W> constexpr int wrap() { W w(4); return w.v; }
template <class T> struct Outer { template <class U> struct In { U u; constexpr In(U x) : u(x) {} }; };
template <class T> constexpr int inner() { typename Outer<T>::In i(5); return i.u; }
int main() { return wrap<Box>() == 4 && inner<int>() == 5 ? 0 : 1; }
