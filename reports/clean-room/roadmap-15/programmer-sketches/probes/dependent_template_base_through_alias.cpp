struct Add { template <class S> struct apply { int a = 1; }; };
template <class K, class S> using apply_t = typename K::template apply<S>;
template <class K> struct D : apply_t<K, int> {};
int main() { D<Add> d; return d.a - 1; }
