template <class R> struct Wrap { template <class S> struct skill { int v = 1; }; };
template <template <class> class K> struct Use { K<int> k; };
int main() { Use<Wrap<char>::skill> u; return u.k.v - 1; }
