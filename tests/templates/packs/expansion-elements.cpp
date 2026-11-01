// tidal-rules: PACK-5
// tidal-expect: accept
// tidal-note: Each copy of the pattern takes one element: a type, a value, a template, or a function parameter. Two packs in one pattern pair up by position.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
template <class A, class B> struct Pair {};
template <class T> struct Wrap {};
template <class T> struct Keep {};
template <class... T> using Pointers = List<T*...>;
static_assert(same<Pointers<int, char>, List<int*, char*>>);
template <int... N> using Doubled = List<int[N * 2]...>;
static_assert(same<Doubled<1, 3>, List<int[2], int[6]>>);
template <template <class> class... W> using Applied = List<W<int>...>;
static_assert(same<Applied<Wrap, Keep>, List<Wrap<int>, Keep<int>>>);
template <class... A> struct Outer {
  template <class... B> using Zip = List<Pair<A, B>...>;
};
static_assert(same<Outer<int, char>::Zip<long, bool>, List<Pair<int, long>, Pair<char, bool>>>);
template <class... T> constexpr int squares(T... x) { return ((x * x) + ... + 0); }
static_assert(squares(1, 2, 3) == 14);
