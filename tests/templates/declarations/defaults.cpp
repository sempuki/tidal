// tidal-rules: TPL-13
// tidal-expect: accept
// tidal-note: Each kind of parameter can have a default, and a default can use earlier parameters.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct List {};
template <class T = int, int N = sizeof(T), template <class> class C = List> struct Config {
  using type = T;
  static constexpr int size = N;
  using container = C<T>;
};
static_assert(same<Config<>::type, int>);
static_assert(Config<char>::size == 1);
static_assert(same<Config<long>::container, List<long>>);
