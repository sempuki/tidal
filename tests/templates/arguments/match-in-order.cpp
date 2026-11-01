// tidal-rules: TARG-5
// tidal-expect: accept
// tidal-note: Each argument goes to the parameter at the same position, and its kind matches the parameter's kind.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct List {};
template <class T, int N, template <class> class C> struct Mix {
  using type = T;
  static constexpr int size = N;
  using container = C<T>;
};
using M = Mix<char, 4, List>;
static_assert(same<M::type, char>);
static_assert(M::size == 4);
static_assert(same<M::container, List<char>>);
