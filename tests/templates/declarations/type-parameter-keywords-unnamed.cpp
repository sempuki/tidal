// tidal-rules: TPL-5
// tidal-expect: accept
// tidal-note: An unnamed constant parameter, an unnamed pack, and typename in a template template parameter's own head.
template <int, class...> struct Skip { static constexpr int value = 1; };
template <template <typename, class> class Two> struct Use { static constexpr int value = 2; };
template <class A, typename B> struct Pair {};
static_assert(Skip<3>::value == 1 && Skip<3, int, char>::value == 1);
static_assert(Use<Pair>::value == 2);
