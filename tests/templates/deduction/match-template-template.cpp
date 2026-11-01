// tidal-rules: DED-36
// tidal-expect: accept
// tidal-note: TT takes Box from Box<int>, whose argument holds no template parameter. In the second template, TT takes Box from the template argument of Hold<Box>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class> struct Box {};
template <template <class> class TT> struct Hold {};
template <template <class> class TT> TT<char> remake(TT<int>);
template <template <class> class TT> TT<char> unwrap(Hold<TT>);
static_assert(same<decltype(remake(Box<int>{})), Box<char>>);
static_assert(same<decltype(unwrap(Hold<Box>{})), Box<char>>);
