// tidal-rules: PACK-7
// tidal-expect: accept
// tidal-note: U... belongs to the inner list and expands inside each copy of the outer pattern. T is expanded by the outer expansion only.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
template <class... T> struct Outer {
  template <class... U> struct Inner { using type = List<List<T, U...>...>; };
};
static_assert(same<Outer<int, char>::Inner<long, short>::type,
                   List<List<int, long, short>, List<char, long, short>>>);
static_assert(same<Outer<int>::Inner<>::type, List<List<int>>>);
