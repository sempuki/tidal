template <class S> struct Add { int a = 1; };
template <template <class> class... K> struct L : K<int>... {};
int main() { L<Add> f; return f.a - 1; }
