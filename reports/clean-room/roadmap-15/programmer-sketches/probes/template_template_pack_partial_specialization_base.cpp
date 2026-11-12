template <class S> struct Add { int a = 1; };
template <class S, template <class> class... K> struct Mix {};
template <class S, template <class> class K, template <class> class... R>
struct Mix<S, K, R...> : K<S>, Mix<S, R...> {};
struct T : Mix<T, Add> { };
int main() { T t; return t.a - 1; }
