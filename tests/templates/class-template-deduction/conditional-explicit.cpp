// tidal-rules: CTAD-5 CTAD-17
// tidal-expect: accept
// tidal-note: The guide from the constructor is explicit only when sizeof(T) > 1 for the deduced T. For char the guide is not explicit, so copy-list and copy initialization accept it. Direct-list initialization accepts both.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Narrow { constexpr explicit(sizeof(T) > 1) Narrow(T) {} };
Narrow a = {'c'};
Narrow b = 'c';
Narrow c{1};
static_assert(same<decltype(a), Narrow<char>> && same<decltype(b), Narrow<char>> && same<decltype(c), Narrow<int>>);
