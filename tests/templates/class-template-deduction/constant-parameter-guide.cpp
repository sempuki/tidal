// tidal-rules: CTAD-1 TPL-11
// tidal-expect: accept
// tidal-note: The type of the constant parameter of Hold is the class template Size. The argument 3 deduces Size<long> through the user guide, as for a variable declared Size s = 3.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Size {
  T value;
  constexpr Size(T v) : value(v) {}
};
Size(int) -> Size<long>;
template <Size S> struct Hold { static constexpr auto held = S; };
static_assert(same<decltype(Hold<3>::held), const Size<long>>);
static_assert(Hold<3>::held.value == 3);
static_assert(same<decltype(Hold<'a'>::held), const Size<char>>);
