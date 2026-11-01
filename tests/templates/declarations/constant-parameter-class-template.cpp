// tidal-rules: TPL-11 CTAD-1
// tidal-expect: accept
// tidal-note: The parameter's type is the name of a class template. The argument gives the class template's arguments.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Wrap {
  T value;
  constexpr Wrap(T v) : value(v) {}
};
template <Wrap W> struct Hold { static constexpr auto value = W.value; };
static_assert(same<decltype(Hold<Wrap{3}>::value), const int>);
static_assert(Hold<4>::value == 4);
static_assert(Hold<'a'>::value == 'a');
static_assert(same<decltype(Hold<'a'>::value), const char>);
