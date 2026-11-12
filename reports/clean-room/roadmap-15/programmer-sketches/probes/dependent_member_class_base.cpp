struct Add { struct apply { int a = 1; }; };
template <class K> struct D : K::apply {};
int main() { D<Add> d; return d.a - 1; }
