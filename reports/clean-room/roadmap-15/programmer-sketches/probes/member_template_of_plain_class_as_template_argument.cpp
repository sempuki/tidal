struct Wrap { template <class S> struct skill { int v = 1; }; };
template <template <class> class K> struct Use { K<int> k; };
int main() { Use<Wrap::skill> u; return u.k.v - 1; }
