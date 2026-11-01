// tidal-rules: PACK-6
// tidal-expect: reject
// tidal-error-clang: different lengths
// tidal-error-gcc: mismatched argument pack lengths
// tidal-error-edg: does not have the same number of elements
// tidal-note: A has two elements and B has one, and one expansion names both.
template <class... T> struct List {};
template <class A, class B> struct Pair {};
template <class... A> struct Outer {
  template <class... B> struct Zip { using type = List<Pair<A, B>...>; };
};
using Bad = Outer<int, char>::Zip<long>::type;
