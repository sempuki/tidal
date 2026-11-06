template <class T> struct Box { T v; };
template <template <class> class C, class T> C<T> make(T v) { return C<T>{v}; }
int main() { return make<Box>(4).v == 4 ? 0 : 1; }
