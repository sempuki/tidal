// tidal-rules: DCL-6
// tidal-expect: accept
// tidal-note: An array of arrays of char with the first bound omitted, initialized from string literals.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
char names[][4] = {"ab", "cde"};
static_assert(same<decltype(names), char[2][4]>);
static_assert(sizeof names == 8);
