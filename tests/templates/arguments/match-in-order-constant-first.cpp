// tidal-rules: TARG-5
// tidal-expect: accept
// tidal-note: The constant arguments 1 and 2 go to the first and third parameters, and char goes to the second, by position.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <int N, class T, int M> struct Mix {
  static constexpr int first = N;
  using type = T;
  static constexpr int last = M;
};
using M = Mix<1, char, 2>;
static_assert(M::first == 1 && M::last == 2);
static_assert(same<M::type, char>);
