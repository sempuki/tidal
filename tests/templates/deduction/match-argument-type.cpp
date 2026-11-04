// tidal-rules: DED-11
// tidal-expect: accept
// tidal-note: A constant parameter declared with auto and deduced from a class template argument gets the type of the class template's parameter.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <long N> struct Wide {};
template <char C> struct Letter {};
template <auto N> Tag<decltype(N)> argument_type(Wide<N>);
template <auto C> Tag<decltype(C)> argument_type(Letter<C>);
static_assert(same<decltype(argument_type(Wide<4>{})), Tag<long>>);
static_assert(same<decltype(argument_type(Letter<'x'>{})), Tag<char>>);
