template <class S> struct Add {};
template <template <class> class... K> struct L { static constexpr int n = sizeof...(K); };
template <template <class> class... K> struct F { static constexpr int n = L<K...>::n; };
static_assert(F<Add, Add>::n == 2);
int main() { return 0; }
