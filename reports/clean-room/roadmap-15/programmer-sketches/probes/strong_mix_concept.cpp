template <class S> struct Add { friend constexpr S operator+(const S& a, const S&) { return a; } };
template <class S, template <class> class... K> struct Mix {};
template <class S, template <class> class K, template <class> class... R>
struct Mix<S, K, R...> : K<S>, Mix<S, R...> {};
template <class T, template <class> class... Skills>
struct Strong : Mix<Strong<T, Skills...>, Skills...> { T v{}; };
using M = Strong<int, Add>;
template <class S> concept addable = requires(const S& a) { a + a; };
static_assert(addable<M>);
int main() { return 0; }
