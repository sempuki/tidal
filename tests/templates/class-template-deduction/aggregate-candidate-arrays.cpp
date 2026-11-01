// tidal-rules: CTAD-9
// tidal-expect: accept
// tidal-note: A nested braced list for an array member gives its bound, and a string literal gives the element type without const and a bound that counts the null character.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T, int N> struct Array { T values[N]; };
Array numbers{{1, 2, 3}};
Array text{"abc"};
static_assert(same<decltype(numbers), Array<int, 3>> && same<decltype(text), Array<char, 4>>);
