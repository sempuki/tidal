struct Add { template <class S> struct apply { int a = 1; }; };
template <class K> struct D { typename K::template apply<int> m; };
int main() { D<Add> d; return d.m.a - 1; }
