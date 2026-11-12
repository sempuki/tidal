struct Add { template <class S> struct apply { int a = 1; }; };
template <class K> struct D : K::template apply<int> {};
int main() { D<Add> d; return d.a - 1; }
